use rand::Rng;

pub struct Lobby {
  id: String,
}

impl Lobby {
  pub fn new() -> Self {
    use rand::thread_rng;
    use rand::distributions::Alphanumeric;

    Self {
      id: thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect(),
    }
  }
}