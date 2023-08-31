import { greet } from 'wasmcon2023:greet/interface';

const greetInterface = {
    greet() {
        return greet() + " and JavaScript!";
    }
};

export { greetInterface as 'interface' }
