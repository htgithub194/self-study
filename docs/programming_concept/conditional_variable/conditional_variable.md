# Conditional Variable

### Defination

* A Synchronization primative.
    * Alow thread to wait for a specific condition
    * Thread goes to sleep when waiting


* What is the difference between ConditionVariable vs Mutext?
    * Mutext:
        * Thread waits for Mutex to be unlocked
    * ConditionVariable:
        * Thread waits for a notification


* The difference is seem not much clear. Let's take an example of a channel which will be used by multiple threads.

* The channel consits of a Queue to hold a shared data.
    * A thread can push data to the channel.
    * Another thread can read data from channel.


* We can think of using mutex to protect the shared data. At a given point of time, only 1 thread can have access and modify the shared data.


![1st_design_channel](images/1st_design_channel.drawio.svg "1st_design_channel")


* The 1st design has a problem:
    * The consume thread has no way to know if the channel is empty or not.
    * So, consume thread might try to acquire the Mutex lock for nothing (in case queue is empty),
    * And, consume thread also has to release the Mutex lock, so the produce thread can push data in queue.
    * It means, the consume thread has to acquire the lock, see nothing in the queue, and release it, and then re-acquire, ...


![problem](images/problem.drawio.svg "problem")


* Conditional Variable comes to the rescue. Let's imagine the scenario:

    | Consume thread    | | Produce thread |
    | :-------- | ------- | -------: |
    | acquire Mutex lock  | | |
    | see nothing in queue | | |
    | setups a Conditional Variable | | |
    | Release mutex | | |
    | goes to sleep | | |
    |  | | acquire Mutex lock |
    |  | | push data on queue |
    |  | | release Mutex lock |
    |  | | use the Conditional Variable to notify to Consume Thread |
    | wakeup by Conditional Variable | |  |
    | acquire Mutex lock | |  |
    | read from queue | |  |


![2nd_design_channel](images/2nd_design_channel.drawio.svg "2nd_design_channel")
