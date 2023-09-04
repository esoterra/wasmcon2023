from proxy_greeter import imports, exports
from proxy_greeter.imports import interface

class Interface(exports.Interface):
    def greet(self) -> str:
        return interface.greet() + " and Python 🐍"
