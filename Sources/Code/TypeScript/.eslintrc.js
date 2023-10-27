module.exports = {
  root: true,
  ignorePatterns: [".eslintrc.js"],
  env: { browser: true, commonjs: true, es6: true },
  plugins: ["@typescript-eslint"],
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:@typescript-eslint/recommended-requiring-type-checking",
    "plugin:prettier/recommended",
  ],
  parserOptions: {
    tsconfigRootDir: __dirname,
    project: ["./tsconfig.json"],
    ecmaVersion: 2022,
    sourceType: "module",
    ecmaFeatures: { jsx: true },
    jsxPragma: "FSComponent",
  },
  rules: {
    "no-console": ["warn", { allow: ["warn", "error", "time", "timeEnd", "info"] }],
    "@typescript-eslint/func-call-spacing": ["warn"],
    "@typescript-eslint/no-shadow": "warn",
    "@typescript-eslint/ban-ts-comment": ["error", { "ts-ignore": "allow-with-description" }],
    "@typescript-eslint/explicit-module-boundary-types": "off",
    "@typescript-eslint/no-empty-function": ["error", { allow: ["arrowFunctions"] }],
    "@typescript-eslint/no-explicit-any": "error",
    "no-unused-vars": "off",
    "@typescript-eslint/no-unused-vars": [
      "warn",
      {
        vars: "all",
        varsIgnorePattern: "^_",
        args: "after-used",
        argsIgnorePattern: "^_",
        ignoreRestSiblings: true,
        destructuredArrayIgnorePattern: "^_",
      },
    ],
    "@typescript-eslint/no-misused-promises": [
      "warn",
      {
        checksConditionals: true,
        checksVoidReturn: false,
        checksSpreads: true,
      },
    ],
    "@typescript-eslint/interface-name-prefix": "off",
    "@typescript-eslint/explicit-function-return-type": "off",
  },
}
