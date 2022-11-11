from celery import Celery

app = Celery('tasks', broker="amqp://127.0.0.1:5672")


@app.task(queue="python-queue")
def say_hello():
    print('Hello from python')
    queue_rust_task.delay()


@app.task(name="rust_say_hello", queue="rust-queue")
def rust_say_hello():
    raise NotImplementedError


@app.task(queue="python-queue")
def queue_rust_task():
    return rust_say_hello.delay()
