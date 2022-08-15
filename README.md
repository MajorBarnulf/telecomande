# telecomande

> note: the typo is voluntary, it makes it easier to find.

## Description

A small crate providing a primitive for the execution of asynchronous tasks by processors through commands.


## Example

```rs
#[tokio::test]
async fn example() {
    use telecomande::Executor;

    // the commands you will send to the processor.
    #[derive(Debug)]
    pub enum Command {
        Greet,
        Say(String),
    }

    // the processor that handles commands.
    pub struct Proc {
        greeting: String,
    }
    #[telecomande::async_trait]
    impl telecomande::Processor for Proc {
        type Command = Command;
        type Error = ();
        async fn handle(&mut self, command: Self::Command) -> Result<(), ()> {
            match command {
                Command::Greet => println!("{}", self.greeting),
                Command::Say(text) => println!("{text}"),
            };
            Ok(())
        }
    }

    // launches an async task to run the processor when it receives commands.
    let handle = telecomande::SimpleExecutor::new(Proc {
        greeting: "Hello".into(),
    })
    .spawn();

    // remotes can be clonned and passed between threads.
    let remote = handle.remote();
    remote.send(Command::Greet).unwrap();
    remote.send(Command::Say("telecomande".into())).unwrap();

    // output:
    // Hello
    // telecomande

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    drop(handle);
}
```
