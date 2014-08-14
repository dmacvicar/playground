import time
from tornado.wsgi import WSGIContainer
from tornado.ioloop import IOLoop, PeriodicCallback
from tornado.web import FallbackHandler, RequestHandler, Application
from tornado.websocket import WebSocketHandler
import tornado.options
tornado.options.parse_command_line()
from minimanager import app

class MainHandler(RequestHandler):
  def get(self):
    self.write("This message comes from Tornado ^_^")

class SaltEventsProxyHandler(WebSocketHandler):
    clients = set()

    def open(self):
        SaltEventsProxyHandler.clients.add(self)

    def on_close(self):
        SaltEventsProxyHandler.clients.remove(self)

    @staticmethod
    def echo_now():
        for client in SaltEventsProxyHandler.clients:
            client.write_message(time.ctime())


tr = WSGIContainer(app)

application = Application([
    (r"/now", SaltEventsProxyHandler),
  (r".*", FallbackHandler, dict(fallback=tr)),
], debug=True)

PeriodicCallback(SaltEventsProxyHandler.echo_now,1000).start()

if __name__ == "__main__":
  application.listen(5000)
  IOLoop.instance().start()
