"""Quiet loopback-only server for browser validation."""
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer

class Handler(SimpleHTTPRequestHandler):
    def log_message(self, format, *args):
        if len(args)>1 and str(args[1]) not in ('200','304'):
            super().log_message(format,*args)

ThreadingHTTPServer(('127.0.0.1',8173),Handler).serve_forever()
