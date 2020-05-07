module.exports = {
    plugins: ["prettier"],
    extends: ["eslint:recommended", "prettier"],
    parserOptions: {
        ecmaVersion: 2015
    },
    env: {
        es6: true,
        node: true
    },
    rules: {
        curly: ["warn", "all"],
        "no-empty": "warn",
        "no-prototype-builtins": "warn",
        "no-redeclare": "warn",
        "no-undef": "warn",
        "no-unused-vars": "warn",
        "no-useless-escape": "warn",
        "prettier/prettier": "warn"
    }
};
