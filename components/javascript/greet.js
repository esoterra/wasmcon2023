import { greet } from 'wasmcon2023:greet/interface@0.1.0';

const greetInterface = {
    greet() {
        return greet() + " and JavaScript!";
    }
};

export { greetInterface as 'interface0.1.0' }
