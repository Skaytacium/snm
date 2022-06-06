mod greater {
	use crate::typings::Entry;
	use crate::typings::Lts;

	#[test]
	fn greater() {
		let a = Entry {
			version: "18.3.0".to_string(),
			lts: Lts::False(false),
		};
		let b = Entry {
			version: "16.3.0".to_string(),
			lts: Lts::False(false),
		};
		assert!(a > b)
	}

	#[test]
	fn greater_minor() {
		let a = Entry {
			version: "18.3.0".to_string(),
			lts: Lts::False(false),
		};
		let b = Entry {
			version: "18.2.0".to_string(),
			lts: Lts::False(false),
		};
		assert!(a > b)
	}

	#[test]
	fn greater_patch() {
		let a = Entry {
			version: "18.3.1".to_string(),
			lts: Lts::False(false),
		};
		let b = Entry {
			version: "18.3.0".to_string(),
			lts: Lts::False(false),
		};
		assert!(a > b)
	}
}

mod smaller {
	use crate::typings::Entry;
	use crate::typings::Lts;

	#[test]
	fn smaller() {
		let a = Entry {
			version: "14.3.0".to_string(),
			lts: Lts::False(false),
		};
		let b = Entry {
			version: "16.3.0".to_string(),
			lts: Lts::False(false),
		};
		assert!(a < b)
	}

	#[test]
	fn smaller_minor() {
		let a = Entry {
			version: "18.1.0".to_string(),
			lts: Lts::False(false),
		};
		let b = Entry {
			version: "18.2.0".to_string(),
			lts: Lts::False(false),
		};
		assert!(a < b)
	}

	#[test]
	fn smaller_patch() {
		let a = Entry {
			version: "18.3.1".to_string(),
			lts: Lts::False(false),
		};
		let b = Entry {
			version: "18.3.3".to_string(),
			lts: Lts::False(false),
		};
		assert!(a < b)
	}
}
mod equal {
	use crate::typings::Entry;
	use crate::typings::Lts;

	#[test]
	fn equal() {
		let a = Entry {
			version: "18.3.0".to_string(),
			lts: Lts::False(false),
		};
		let b = Entry {
			version: "18.3.0".to_string(),
			lts: Lts::False(false),
		};
		assert!(a == b)
	}

	#[test]
	fn equal_minor() {
		let a = Entry {
			version: "18.3.0".to_string(),
			lts: Lts::False(false),
		};
		let b = Entry {
			version: "18.3.0".to_string(),
			lts: Lts::False(false),
		};
		assert!(a == b)
	}

	#[test]
	fn equal_patch() {
		let a = Entry {
			version: "18.3.0".to_string(),
			lts: Lts::False(false),
		};
		let b = Entry {
			version: "18.3.0".to_string(),
			lts: Lts::False(false),
		};
		assert!(a == b)
	}
}

// mod netio {
// 	use crate::util::netio::recurse_move_dir;
// 	use std::path;

// 	#[test]
// 	fn recurse_move_dir_test() {
// 		if let Err(e) = recurse_move_dir(
// 			&path::Path::new("C:/Users/Sid/.snm/versions/v18.3.0/node-v18.3.0-win-x64")
// 				.to_path_buf(),
// 			&path::Path::new("C:/Users/Sid/.snm/versions/v18.3.0").to_path_buf(),
// 		) {
// 			println!("{}", e)
// 		}
// 	}
// }
