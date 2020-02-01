%%%
**here is a notedown comment**
-
-
%%%


{database}
server = "192.168.1.1"
ports = [8001, 8001, 8002]
connection_max = 5000
enabled = true


{servers}
{servers.alpha}

pw = "力微任重久神疲"
{servers.beta}
ip = ip`192.168.256.1
pw = "再竭衰庸定不支"
{servers.meta}
ip = [
    $servers/alpha/ip,
    $servers/beta/ip,
]
pw = $servers/*/pw

{clients}
data = [["gamma", "delta"], [1, 2]]
hosts = [
    "alpha",
    "omega", % Line breaks are OK when inside arrays
]


{String}
singleline = "I'm a string. \"You can quote me\". Name\tJos\u00E9\nLocation\tSF."
multiline = """
\b     - backspace       (U+0008)
\t     - tab             (U+0009)
\n     - linefeed        (U+000A)
\f     - form feed       (U+000C)
\r     - carriage return (U+000D)
\"     - quote           (U+0022)
\/     - slash           (U+002F)
\\     - backslash       (U+005C)
\u1234 - unicode         (U+1234)
"""


literal = '''
\b     - backspace       (U+0008)
\t     - tab             (U+0009)
\n     - linefeed        (U+000A)
\f     - form feed       (U+000C)
\r     - carriage return (U+000D)
\"     - quote           (U+0022)
\/     - slash           (U+002F)
\\     - backslash       (U+005C)
\u1234 - unicode         (U+1234)
'''


{Number}
int1 =+99
int2 = 42
int3 = 0
int4 =-17
int5 = 1_000
int6 = 5_349_221
int7 = 1_2_3_4_5     % VALID but discouraged
% fractional
dec1 =+1.0
dec2 =-0.0_2
dec3 =+5_0.
dec4 = 3.1415
dec5 =-0.01
dec6 = 224_617.445_991_228
% infinity
sf1 = infinity % positive infinity
sf2 =+Infinity % positive infinity
sf3 =-INFINITY % negative infinity
% not a number
sf4 = nan % actual sNaN/qNaN encoding is implementation specific
sf5 =+NaN % same as `nan`
sf6 =-NAN % valid, actual encoding is implementation specific


{Array}
arr1 = [1, 2, 3]
arr2 = ["red", "yellow", "green"]
arr3 = [[1, 2], [3, 4, 5]]
arr4 = ["all", 'strings', """are the same""", '''type''']
arr5 = [
    [1, 2]
    ["a", "b", "c"]
]
arr6 = [1, 2.0]


{fruit}
name = "apple"
inlineTable = [
    { x = 1, y = 2, z = 3 },
    { x = 7, y = 5, z = 9 },
    { x = 2, y = 4, z = 8 },
]


[fruit.variety]
* name = "red delicious"
* name = "granny smith"
[fruit.physical]
* color = "red"
* shape = "round"
> [1, 2, 3]
