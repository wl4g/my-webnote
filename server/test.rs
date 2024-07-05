// use serde::Deserialize;

// #[derive(Deserialize, Debug)]
// struct Request {
//   // Use the result of a function as the default if "resource" is
//   // not included in the input.
//   #[serde(default = "default_resource")]
//   resource: Option<String>,

//   // Use the type's implementation of std::default::Default if
//   // "timeout" is not included in the input.
//   #[serde(default)]
//   timeout: Timeout,

//   // Use a method from the type as the default if "priority" is not
//   // included in the input. This may also be a trait method.
//   #[serde(default = "Priority::lowest")]
//   priority: Priority,
// }

// fn default_resource() -> Option<String> {
//   Some("/".to_string())
// }

// /// Timeout in seconds.
// #[derive(Deserialize, Debug)]
// struct Timeout(u32);
// impl Default for Timeout {
//   fn default() -> Self {
//     Timeout(30)
//   }
// }

// #[derive(Deserialize, Debug)]
// enum Priority {
//   ExtraHigh,
//   High,
//   Normal,
//   Low,
//   ExtraLow,
// }
// impl Priority {
//   fn lowest() -> Self {
//     Priority::ExtraLow
//   }
// }

// fn main() {
//   let contents =
//     r#"
// - resource: "/users"
//   timeout: 30
//   priority: High
// - resource: "/users/:id"
//   timeout: 11
//   priority: Low
// "#;

//   let requests: Vec<Request> = serde_yaml::from_str(&contents).unwrap();

//   // The first request has resource="/users", timeout=30, priority=ExtraLow
//   println!("{:?}", requests[0]);

//   // The second request has resource="/", timeout=5, priority=High
//   println!("{:?}", requests[1]);
// }
