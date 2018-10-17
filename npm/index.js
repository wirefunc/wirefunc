var binwrap = require("binwrap");
var path = require("path");

var packageInfo = require(path.join(__dirname, "package.json"));
var version = packageInfo.version;
var binaryName = "wf";
var root = "https://github.com/wirefunc/wirefunc/releases/download/wf" + version;

module.exports = binwrap({
  dirname: __dirname,
  binaries: [binaryName],
  urls: {
    "darwin-x64": root + "/mac-x64.tgz",
    "linux-x64": root + "/linux-x64.tgz",
    "win32-x64": root + "/win-x64.zip"
  }
});
