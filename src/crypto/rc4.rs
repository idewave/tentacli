pub struct RC4 {
    i: u8,
    j: u8,
    pub state: [u8; 256],
}

impl RC4 {
    pub fn new(key: Vec<u8>) -> Self {
        assert!(key.len() >= 1 && key.len() <= 256);

        let mut state = [0u8; 256];
        let mut j: u8 = 0;

        for (i, x) in state.iter_mut().enumerate() {
            *x = i as u8;
        }
        for i in 0..256 {
            j = j.wrapping_add(state[i]).wrapping_add(key[i % key.len()]);
            state.swap(i, j as usize);
        }

        Self {
            i: 0,
            j: 0,
            state,
        }
    }

    // prga
    pub fn next(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.state[self.i as usize]);
        self.state.swap(self.i as usize, self.j as usize);
        self.state[(self.state[self.i as usize].wrapping_add(self.state[self.j as usize])) as usize]
    }

    pub fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
        let mut encrypted = Vec::new();
        for (_, x) in data.iter().enumerate() {
            encrypted.push(x ^ self.next());
        }

        encrypted
    }
}