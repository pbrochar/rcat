pub struct Commande {
	pub path: Vec<String>,
	pub n_option: bool,
	pub e_option: bool,
	pub is_valid: bool,
	pub unknown_option: Option<char>,
}

impl Commande {
	pub fn set_n_option(&mut self, value: bool) {
			self.n_option = value;
	}
	pub fn set_e_option(&mut self, value: bool) {
			self.e_option = value;
	}
	pub fn set_validity(&mut self, value: bool, option: char) {
			self.is_valid = value;
			self.unknown_option = Some(option);
	}
	pub fn add_path(&mut self, path: &String) {
			self.path.push(path.to_string());
	}
}