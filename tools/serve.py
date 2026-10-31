"""Quiet loopback-only server for browser validation."""
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer

class Handler(SimpleHTTPRequestHandler):
    def log_message(self, format, *args):
        if len(args)>1 and str(args[1]) not in ('200','304'):
            super().log_message(format,*args)

class Server(ThreadingHTTPServer):
    # A page's hundreds of module requests at once overflow the default backlog of 5:
    # the connections past it are reset and the page never comes up.
    request_queue_size=256

Server(('127.0.0.1',8173),Handler).serve_forever()
