module.exports = {
  globals: {
    "ts-jest": {
      tsConfig: "tsconfig.json"
    }
  },
  moduleFileExtensions: [
    "ts",
    "js"
  ],
  transform: {
    "^.+\\.(ts|tsx)$": "ts-jest"
  },
  testMatch: [
    "**/test/**/*.test.(ts|js)",
    "**/__test__/**/*.test.(ts|js)"
  ],
  testPathIgnorePatterns: [
    "/node_modules/"
  ],
  testEnvironment: "node"
};
