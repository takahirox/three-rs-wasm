"""Quiet loopback-only server for browser validation.

python3 tools/serve.py [port] [directory]: the repository on port 8173 by
default; the published-gallery check serves the built site
( playwright.pages.config.js ).
"""
import sys
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer

class Handler(SimpleHTTPRequestHandler):
    def log_message(self, format, *args):
        if len(args)>1 and str(args[1]) not in ('200','304'):
            super().log_message(format,*args)

class Server(ThreadingHTTPServer):
    # A page's hundreds of module requests at once overflow the default backlog of 5:
    # the connections past it are reset and the page never comes up.
    request_queue_size=256

port=int(sys.argv[1]) if len(sys.argv)>1 else 8173
directory=sys.argv[2] if len(sys.argv)>2 else None
Server(('127.0.0.1',port),partial(Handler,directory=directory)).serve_forever()
