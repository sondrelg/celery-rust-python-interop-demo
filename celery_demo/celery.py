from enum import Enum

from celery import Celery

app = Celery('tasks', broker="amqp://127.0.0.1:5672")


class Queue(str, Enum):
    Python = "python-queue"
    Rust = "rust-queue"
    Go = "go-queue"


class RustTasks(str, Enum):
    SayHello = "say_hello"


@app.task(queue="python-queue")
def queue_rust_task():
    print('Queueing rust task')
    app.send_task(name=RustTasks.SayHello, queue=Queue.Rust)


@app.task(queue="python-queue")
def say_hello():
    print('Hello from python')
