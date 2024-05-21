const STEP_LIMIT: usize = 100_000;
const STARTING_POINT: usize = 1;
const ENDING_POINT: usize = u16::MAX as usize;

#[tokio::main(flavor = "multi_thread", worker_threads = 1028)]
async fn main() {
	let mut output: String = String::new();

	println!("Range: {STARTING_POINT} to {ENDING_POINT}");
	println!("Step limit: {STEP_LIMIT}");
	println!("Starting...");

	let start_time = std::time::Instant::now();

	for i in STARTING_POINT..=ENDING_POINT {
		match tokio::spawn(collatz_conjecture(i)).await.unwrap() {
			Some(steps) => {
				output.push_str(
					format!("{i} converges within {steps} steps\n").as_str()
				);
			}
			None => {
				output.push_str(
					format!("{i} failed to converge within {STEP_LIMIT} steps\n").as_str()
				);
			}
		}
	}

        println!("{}\n...\n{}", output.split('\n').collect::<Vec<&str>>()[0], output.split('\n').collect::<Vec<&str>>()[output.split('\n').collect::<Vec<&str>>().len() - 2]);
        

	if !std::path::PathBuf::from("./out").exists() {
		println!("Creating \"./out\" directory...");
		tokio::fs::create_dir("./out").await
			.expect("Failed to create output directory");
	}

	println!("Saving output to \"./out/out.txt\"...");
	tokio::fs::write("./out/out.txt", output).await
		.expect("Failed to save output to file");

	println!(
		"Took {}.{:>03} seconds",
		start_time.elapsed().as_secs(),
		start_time.elapsed().subsec_millis()
	);
}

#[allow(clippy::unused_async)]
async fn collatz_conjecture(starting_number: usize) -> Option<usize> {
	let mut numbers: Vec<u128> = Vec::new();
	let mut current = starting_number;

	for _ in 0..STEP_LIMIT {
		if current % 2 == 0 {
			numbers.push(current as u128);
			current /= 2;
		} else if current == 1 {
			numbers.push(current as u128);
			break
		} else {
			numbers.push(current as u128);
			current = current * 3 + 1;
		}
	}

	if numbers.len() < STEP_LIMIT {
		Some(numbers.len())
	} else {
		None
	}
}