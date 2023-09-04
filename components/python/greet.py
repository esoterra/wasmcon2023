from greeter_adapter import imports, exports

class Interface(exports.Interface):
    def greet(self) -> str:
        return imports.interface.greet() + " and Python 🐍"
