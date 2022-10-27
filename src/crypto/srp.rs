use num_bigint::{BigInt, Sign, ToBigInt};
use sha1::{Digest};

pub struct Srp {
    modulus: BigInt,
    generator: BigInt,
    private_ephemeral: BigInt,
    public_ephemeral: BigInt,
    server_ephemeral: BigInt,
    session_key: Vec<u8>,
}

// public methods
impl Srp {
    pub fn new(n: Vec<u8>, g: Vec<u8>, server_ephemeral: Vec<u8>) -> Self {
        let private_ephemeral: [u8; 19] = rand::random();

        let modulus = BigInt::from_bytes_le(Sign::Plus, &n);
        let generator = BigInt::from_bytes_le(Sign::Plus, &g);

        let private_ephemeral = BigInt::from_bytes_le(Sign::Plus, &private_ephemeral);
        let public_ephemeral = generator.modpow(&private_ephemeral, &modulus);
        let server_ephemeral = BigInt::from_bytes_le(Sign::Plus, &server_ephemeral);

        Self {
            modulus,
            generator,
            private_ephemeral,
            public_ephemeral,
            server_ephemeral,
            session_key: Vec::new(),
        }
    }

    pub fn public_ephemeral(&mut self) -> Vec<u8> {
        self.public_ephemeral.to_bytes_le().1
    }

    pub fn session_key(&mut self) -> Vec<u8> {
        // I believe this can be optimized
        self.session_key.to_vec()
    }

    pub fn calculate_proof<D>(
        &mut self,
        account: &str,
        password: &str,
        salt: &[u8]
    ) -> Vec<u8>
    where
        D: Digest,
    {
        let account = account.to_uppercase();
        let password = password.to_uppercase();

        self.session_key = self.calculate_session_key::<D>(&account, &password, salt);

        D::new()
            .chain(self.calculate_xor_hash::<D>())
            .chain(Self::calculate_account_hash::<D>(&account))
            .chain(&salt)
            .chain(&self.public_ephemeral.to_bytes_le().1)
            .chain(&self.server_ephemeral.to_bytes_le().1)
            .chain(&self.session_key)
            .finalize()
            .to_vec()
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
        let n_hash = D::new().chain(&self.modulus.to_bytes_le().1).finalize();
        let g_hash = D::new().chain(&self.generator.to_bytes_le().1).finalize();

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
            .chain(&self.public_ephemeral.to_bytes_le().1)
            .chain(&self.server_ephemeral.to_bytes_le().1)
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

    fn calculate_session_key<D>(&mut self, account: &str, password: &str, salt: &[u8]) -> Vec<u8>
    where
        D: Digest,
    {
        let x = self.calculate_x::<D>(account, password, salt);
        let verifier = self.generator.modpow(
            &x,
            &self.modulus,
        );
        Self::calculate_interleaved::<D>(
            self.calculate_s::<D>(x, verifier)
        )
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

        let hashed1 = D::new().chain(&part1).finalize();
        let hashed2 = D::new().chain(&part2).finalize();

        let mut session_key = Vec::new();
        for (index, _) in hashed1.iter().enumerate() {
            session_key.push(hashed1[index]);
            session_key.push(hashed2[index]);
        }

        session_key
    }
}