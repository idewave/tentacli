use num_bigint::{BigInt, Sign, ToBigInt};
use sha1::{Digest, Sha1};

#[derive(Debug, Default)]
pub struct Srp {
    pub session_key: Vec<u8>,
    modulus: BigInt,
    generator: BigInt,
    private_ephemeral: BigInt,
    public_ephemeral: BigInt,
    server_ephemeral: BigInt,
    salt: [u8; 32],
    client_proof: Option<[u8; 20]>,
}

impl Srp {
    pub fn init(&mut self, n: &[u8], g: &[u8], server_ephemeral: &[u8; 32], salt: [u8; 32]) {
        self.modulus = BigInt::from_bytes_le(Sign::Plus, n);
        self.generator = BigInt::from_bytes_le(Sign::Plus, g);

        self.private_ephemeral = {
            let private_ephemeral: [u8; 19] = rand::random();
            BigInt::from_bytes_le(Sign::Plus, &private_ephemeral)
        };

        self.public_ephemeral = self
            .generator
            .modpow(&self.private_ephemeral, &self.modulus);
        self.server_ephemeral = BigInt::from_bytes_le(Sign::Plus, server_ephemeral);
        self.salt = salt;
    }

    pub fn public_ephemeral(&mut self) -> [u8; 32] {
        Self::pad_to_32_bytes(self.public_ephemeral.to_bytes_le().1)
    }

    pub fn calculate_proof(&mut self, account: &str) -> [u8; 20] {
        let result = Sha1::new()
            .chain(self.calculate_xor_hash())
            .chain(Self::calculate_account_hash(account))
            .chain(self.salt)
            .chain(self.public_ephemeral.to_bytes_le().1)
            .chain(self.server_ephemeral.to_bytes_le().1)
            .chain(&self.session_key)
            .finalize()
            .to_vec();

        let mut output = [0u8; 20];
        output.copy_from_slice(&result);

        self.client_proof = Some(output);

        output
    }

    pub fn calculate_session_key(&mut self, account: &str, password: &str) {
        let salt = self.salt;
        let x = self.calculate_x(account, password, &salt);
        let verifier = self.generator.modpow(&x, &self.modulus);

        let mut session_key = Self::calculate_interleaved(self.calculate_s(x, verifier));

        // sometimes session key has trailing 0, but on mangos side there was no trailing zero
        // so, actually, session key can be less than 40 bytes
        while let Some(&0) = session_key.last() {
            session_key.truncate(session_key.len() - 1);
        }

        self.session_key = session_key;
    }

    pub fn validate_proof(&mut self, server_proof: [u8; 20]) -> bool {
        let client_proof = {
            let hasher = Sha1::new();

            let result = hasher
                .chain(self.public_ephemeral())
                .chain(self.client_proof.unwrap())
                .chain(self.session_key.clone())
                .finalize();

            let mut hashed_proof = [0u8; 20];
            hashed_proof.copy_from_slice(&result);
            hashed_proof
        };

        client_proof == server_proof
    }

    fn calculate_account_hash(account: &str) -> Vec<u8> {
        Sha1::new().chain(account.as_bytes()).finalize().to_vec()
    }

    fn calculate_xor_hash(&mut self) -> Vec<u8> {
        let n_hash = Sha1::new().chain(self.modulus.to_bytes_le().1).finalize();
        let g_hash = Sha1::new().chain(self.generator.to_bytes_le().1).finalize();

        let mut xor_hash = Vec::new();
        for (index, value) in g_hash.iter().enumerate() {
            xor_hash.push(value ^ n_hash[index]);
        }

        xor_hash
    }

    fn calculate_x(&mut self, account: &str, password: &str, salt: &[u8]) -> BigInt {
        let identity_hash = Sha1::new()
            .chain(format!("{}:{}", account, password).as_bytes())
            .finalize()
            .to_vec();

        let x = Sha1::new()
            .chain(salt)
            .chain(identity_hash)
            .finalize()
            .to_vec();

        BigInt::from_bytes_le(Sign::Plus, &x)
    }

    fn calculate_u(&mut self) -> BigInt {
        let u = Sha1::new()
            .chain(self.public_ephemeral.to_bytes_le().1)
            .chain(self.server_ephemeral.to_bytes_le().1)
            .finalize()
            .to_vec();

        BigInt::from_bytes_le(Sign::Plus, &u)
    }

    fn calculate_s(&mut self, x: BigInt, verifier: BigInt) -> BigInt {
        const K: u8 = 3;
        let u = self.calculate_u();
        let mut s = &self.server_ephemeral - K.to_bigint().unwrap() * verifier;
        s = s.modpow(&(&self.private_ephemeral + u * x), &self.modulus);
        s
    }

    fn calculate_interleaved(s: BigInt) -> Vec<u8> {
        let (even, odd): (Vec<_>, Vec<_>) = Self::pad_to_32_bytes(s.to_bytes_le().1)
            .into_iter()
            .enumerate()
            .partition(|(i, _)| i % 2 == 0);

        let part1 = even.iter().map(|(_, v)| *v).collect::<Vec<u8>>();
        let part2 = odd.iter().map(|(_, v)| *v).collect::<Vec<u8>>();

        let hashed1 = Sha1::new().chain(part1).finalize();
        let hashed2 = Sha1::new().chain(part2).finalize();

        let mut session_key = Vec::new();
        for (index, _) in hashed1.iter().enumerate() {
            session_key.push(hashed1[index]);
            session_key.push(hashed2[index]);
        }

        session_key
    }

    fn pad_to_32_bytes(bytes: Vec<u8>) -> [u8; 32] {
        let mut buffer = [0u8; 32];
        buffer[..bytes.len()].copy_from_slice(&bytes);
        buffer
    }
}
