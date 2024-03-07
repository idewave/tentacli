use num_bigint::{BigInt, Sign, ToBigInt};
use sha1::{Digest, Sha1};

#[derive(Debug)]
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

// public methods
impl Srp {
    pub fn new(n: &[u8], g: &[u8], server_ephemeral: &[u8; 32], salt: [u8; 32]) -> Self {
        let private_ephemeral: [u8; 19] = rand::random();

        let modulus = BigInt::from_bytes_le(Sign::Plus, n);
        let generator = BigInt::from_bytes_le(Sign::Plus, g);

        let private_ephemeral = BigInt::from_bytes_le(Sign::Plus, &private_ephemeral);
        let public_ephemeral = generator.modpow(&private_ephemeral, &modulus);
        let server_ephemeral = BigInt::from_bytes_le(Sign::Plus, server_ephemeral);

        Self {
            session_key: Vec::new(),
            modulus,
            generator,
            private_ephemeral,
            public_ephemeral,
            server_ephemeral,
            salt,
            client_proof: None,
        }
    }

    pub fn public_ephemeral(&mut self) -> [u8; 32] {
        self.public_ephemeral.to_bytes_le().1.try_into().unwrap()
    }

    pub fn session_key(&mut self) -> Vec<u8> {
        self.session_key.to_vec()
    }

    pub fn calculate_proof<D>(&mut self, account: &str) -> [u8; 20]
    where
        D: Digest,
    {
        let result = D::new()
            .chain(self.calculate_xor_hash::<D>())
            .chain(Self::calculate_account_hash::<D>(account))
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

    pub fn calculate_session_key<D>(&mut self, account: &str, password: &str)
        where
            D: Digest,
    {
        let salt = self.salt;
        let x = self.calculate_x::<D>(account, password, &salt);
        let verifier = self.generator.modpow(
            &x,
            &self.modulus,
        );

        self.session_key = Self::calculate_interleaved::<D>(
            self.calculate_s::<D>(x, verifier)
        );
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
}

// private methods
impl Srp {
    fn calculate_account_hash<D>(account: &str) -> Vec<u8>
    where
        D: Digest
    {
        D::new()
            .chain(account.as_bytes())
            .finalize()
            .to_vec()
    }

    fn calculate_xor_hash<D>(&mut self) -> Vec<u8>
    where
        D: Digest,
    {
        let n_hash = D::new().chain(self.modulus.to_bytes_le().1).finalize();
        let g_hash = D::new().chain(self.generator.to_bytes_le().1).finalize();

        let mut xor_hash = Vec::new();
        for (index, value) in g_hash.iter().enumerate() {
            xor_hash.push(value ^ n_hash[index]);
        }

        xor_hash
    }

    fn calculate_x<D>(&mut self, account: &str, password: &str, salt: &[u8]) -> BigInt
    where
        D: Digest,
    {
        let identity_hash = D::new()
            .chain(format!("{}:{}", account, password).as_bytes())
            .finalize()
            .to_vec();

        let x = D::new()
            .chain(salt)
            .chain(identity_hash)
            .finalize()
            .to_vec();

        BigInt::from_bytes_le(Sign::Plus, &x)
    }

    fn calculate_u<D>(&mut self) -> BigInt
    where
        D: Digest,
    {
        let u = D::new()
            .chain(self.public_ephemeral.to_bytes_le().1)
            .chain(self.server_ephemeral.to_bytes_le().1)
            .finalize()
            .to_vec();

        BigInt::from_bytes_le(Sign::Plus, &u)
    }

    fn calculate_s<D>(&mut self, x: BigInt, verifier: BigInt) -> BigInt
    where
        D: Digest,
    {
        const K: u8 = 3;
        let u = self.calculate_u::<D>();
        let mut s = &self.server_ephemeral - K.to_bigint().unwrap() * verifier;
        s = s.modpow(
            &(&self.private_ephemeral + u * x),
            &self.modulus,
        );
        s
    }

    fn calculate_interleaved<D>(s: BigInt) -> Vec<u8>
    where
        D: Digest
    {
        let (even, odd): (Vec<_>, Vec<_>) =
            s.to_bytes_le().1
                .into_iter()
                .enumerate()
                .partition(|(i, _)| i % 2 == 0);

        let part1 = even.iter().map(|(_, v)| *v).collect::<Vec<u8>>();
        let part2 = odd.iter().map(|(_, v)| *v).collect::<Vec<u8>>();

        let hashed1 = D::new().chain(part1).finalize();
        let hashed2 = D::new().chain(part2).finalize();

        let mut session_key = Vec::new();
        for (index, _) in hashed1.iter().enumerate() {
            session_key.push(hashed1[index]);
            session_key.push(hashed2[index]);
        }

        session_key
    }
}