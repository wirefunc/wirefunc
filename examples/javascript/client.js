import wirefunc as wf;


// TODO use `if (json.field != null) json.field = defaultVal;` for defaults!
// TODO do an example of renames with fka, like the above.
// TODO attempt packing. Do stuff like storing structs as tuples on the wire:
//      json.users = json.users.map(function(packed) {
//          return {
//              username: packed["a"],
//              email: packed["b"],
//              name: packed["c"]
//          };
//      });
//
// See how it benchmarks! Comparing serialization perf of tuples vs objects is
// pretty easy. See how it goes with gzip. Try a GitHub API payload. Although I
// guess you still kinda need to set up a gzipped server roundtrip to *really*
// know. Maybe try it on the web perf bench to see it on a real low-end device?


/**
  @typedef User_Profile
  @type {object}
  @property {!int} userId
  @property {!Profile} profile
  @property {!Array.<!string>} aliases
 */
verify$user_profile(val) {
  // TODO
}

/**
  @typedef Profile
  @type {object}
  @property {!string} name
  @property {?string} email
  @property {?homepage} homepage
  @property {?string} fullname - deprecated
 */

var wf =
  { nullToUndefined: function nullToUndefined(val) { return val === null ? undefined : val; } };

/**
 * @typedef Params_sendDM
 * @type {object}
 *
 * @callback Response_sendDM
 * @param {!ResponseBody_sendDM}
 */
function sendDM(params) {
  return {
    endpoint: "sendDM"
    verb: "POST",
    body: {
      a: params.name,
      b: params.email,
      c: params.homepage
    },
    callback: function(body) {
      try {
        var json = JSON.parse(body);

        switch (json.a /* Result variant type */) {
          case 1 /* Result.ok */:
            var user_profile /* : User Profile */ = verify$user_profile(json.b);

            return {
              error: null,
              response: {
                variant: "ok",
                content: user_profile
              }
            };
          case 2 /* Result.err */:
            var array_string /* : Array String */ = wf.verifyArray(json.b, wf.verifyString);

            return {
              variant: "err",
              content: array_string
            };

          default:
            throw new Error("Result was: " + json.type);
        }
      } catch (err) {
        return {error: err, response: null};
      }
    }
  };
}

