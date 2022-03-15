export {};

declare global {
  interface String {
    format(...args: string[]): string;
  }
}

String.prototype.format = function (...args) {
  return this.replace(/{(\d+)}/g, function (match, number) {
    return typeof args[number] != 'undefined' ? args[number] : match;
  });
};
