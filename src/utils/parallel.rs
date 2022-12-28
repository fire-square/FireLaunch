//! Parallelise tasks.
//!
//! This struct is used to easily spawn async tasks and limit the number of
//! concurrent futures.

use tokio::task::JoinHandle;

/// Parallelise tasks.
///
/// This struct is used to easily spawn async tasks and limit the number of
/// concurrent futures.
///
/// Tasks shoud have the same return type. Return values are not stored.
///
/// # Example
///
/// ```
/// use tokio::runtime::Runtime;
/// use firesquare_launcher::utils::parallel::Parallelise;
///
/// let rt = Runtime::new().unwrap();
/// rt.block_on(async {
///   // Limit to 10 concurrent tasks
///   let mut parallel = Parallelise::new(Some(10));
///   for i in 0..20 {
///     parallel.push(tokio::spawn(async move {
///       println!("Task {} started", i);
///       tokio::time::sleep(std::time::Duration::from_millis(100)).await;
///       println!("Task {} finished", i);
///     })).await;
///   }
///   // Wait for all tasks to finish
///   parallel.wait().await;
/// })
/// ```
pub struct Parallelise<T> {
	tasks: Vec<JoinHandle<T>>,
	max_tasks: usize,
}

impl<T> Parallelise<T> {
	/// Create a new Parallelise struct.
	///
	/// # Arguments
	///
	/// * `max_tasks` - The maximum number of concurrent tasks. If None, the
	/// number of CPUs will be used.
	pub fn new(max_tasks: Option<usize>) -> Self {
		let max_tasks = max_tasks.unwrap_or(num_cpus::get() * 2);
		Self {
			tasks: Vec::with_capacity(max_tasks),
			max_tasks,
		}
	}

	/// Push a new task to the set.
	///
	/// If the set is full, this function will wait for one of the tasks to
	/// finish before adding the new task.
	pub async fn push(&mut self, task: JoinHandle<T>) {
		loop {
			// If set have less than max_tasks, we can add new task
			if self.tasks.len() < self.max_tasks {
				break;
			}
			// Find finished tasks and remove them
			for (j, task) in self.tasks.iter_mut().enumerate() {
				if task.is_finished() {
					// And remove it from the set
					self.tasks.remove(j);
					break;
				}
			}
			// Check set again
			if self.tasks.len() < self.max_tasks {
				break;
			}
			// Sleep for 5ms to avoid busy waiting
			tokio::time::sleep(std::time::Duration::from_millis(5)).await;
		}
		// Add task to the set
		self.tasks.push(task);
	}

	/// Wait for all tasks to finish.
	///
	/// This function will wait for all tasks to finish before returning.
	pub async fn wait(&mut self) {
		loop {
			// Find finished tasks and remove them
			for (j, task) in self.tasks.iter_mut().enumerate() {
				if task.is_finished() {
					// And remove it from the set
					self.tasks.remove(j);
					break;
				}
			}
			// If set is empty, break
			if self.tasks.is_empty() {
				break;
			}
			// Sleep for 5ms to avoid busy waiting and check again
			tokio::time::sleep(std::time::Duration::from_millis(5)).await;
		}
	}
}

impl Default for Parallelise<()> {
	fn default() -> Self {
		Self::new(None)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_parallelise() {
		let mut parallel = Parallelise::new(Some(10));
		for _ in 0..100 {
			parallel.push(tokio::spawn(async move {})).await;
		}
		parallel.wait().await;
	}
}
