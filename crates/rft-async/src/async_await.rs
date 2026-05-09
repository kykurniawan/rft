#[cfg(test)]
mod test {
    use std::time::Duration;

    use trpl::{Either, Html, StreamExt};

    async fn page_title(url: &str) -> (&str, Option<String>) {
        let response = trpl::get(url).await;
        let response_text = response.text().await;

        let title = Html::parse(&response_text)
            .select_first("title")
            .map(|title| title.inner_html());

        (url, title)
    }

    #[test]
    fn test_async_await() {
        trpl::block_on(async {
            let title_fut_1 = page_title("https://www.google.org/");
            let title_fut_2 = page_title("https://www.microsoft.com/");

            let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

            println!("Returned First: {url}");

            match maybe_title {
                Some(title) => println!("Page Title: '{title}'"),
                None => println!("It had no title."),
            }
        })
    }

    #[test]
    fn create_task_with_spawn() {
        trpl::block_on(async {
            let handle = trpl::spawn_task(async {
                for i in 1..=10 {
                    println!("Hi number {i} from first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            for i in 1..=5 {
                println!("Hi number {i} from second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }

            handle.await.unwrap()
        });
    }

    #[test]
    fn join() {
        trpl::block_on(async {
            let fut_1 = async {
                for i in 1..=10 {
                    println!("Hi number {i} from first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let fut_2 = async {
                for i in 1..=5 {
                    println!("Hi number {i} from second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            trpl::join(fut_1, fut_2).await;
        });
    }

    #[test]
    fn sending_data() {
        trpl::block_on(async {
            let (tx, mut rx) = trpl::channel::<String>();

            let val = String::from("Hello, world!");
            tx.send(val).unwrap();

            let received = rx.recv().await.unwrap();
            println!("Received: {}", received);
        });
    }

    #[test]
    fn sending_series_of_data() {
        trpl::block_on(async {
            let (tx, mut rx) = trpl::channel::<String>();

            let vals = vec![
                String::from("Hello, world!"),
                String::from("Hello, Rust!"),
                String::from("Hello, async!"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }

            while let Some(received) = rx.recv().await {
                println!("Received: {}", received);
            }
        });
    }

    #[test]
    fn sending_series_of_data_sequentially() {
        trpl::block_on(async {
            let (tx, mut rx) = trpl::channel::<String>();

            let tx_fut = async {
                let vals = vec![
                    String::from("Hello, world!"),
                    String::from("Hello, Rust!"),
                    String::from("Hello, async!"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let rx_fut = async {
                while let Some(received) = rx.recv().await {
                    println!("Received: {}", received);
                }
            };

            trpl::join(tx_fut, rx_fut).await;
        });
    }

    #[test]
    fn streaming_data() {
        trpl::block_on(async {
            let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let iter = values.iter().map(|n| n * 2);
            let mut stream = trpl::stream_from_iter(iter);

            while let Some(val) = stream.next().await {
                println!("Received: {}", val);
            }
        });
    }
}
