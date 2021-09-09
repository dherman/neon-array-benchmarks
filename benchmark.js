module.exports = function benchmark(thunk) {
  const start = process.hrtime();
  const result = thunk();
  const time = process.hrtime(start);
  return { result, time };
};
