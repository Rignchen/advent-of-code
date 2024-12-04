import Foundation

func getInputs() -> [String] {
	// get the file name from the command line
	if CommandLine.argc < 2 {
		print("Usage: swift script_1.swift <input_file>")
		return []
	}
	let file = CommandLine.arguments[1]

	let contents = getInputValue(name: file)
	let lines = contents.split(separator: "\n")
	return lines.map { String($0) }
}

func main() {
	let lines = getInputs()
	if lines.isEmpty {
		return
	}

    var jmp_nop = 0
    var result: (Int, Bool)
    repeat {
        result = runProgram(lines: lines, nop_jmp_to_change: jmp_nop)
        jmp_nop += 1
    } while result.1
	print(result.0)
}

func runProgram(lines: [String], nop_jmp_to_change: Int) -> (Int, Bool) {
	var acc = 0
	var pos = 0
    var nop_jmp = 0
    var has_break = false

	// instruction argument
	let instructions = lines.map { $0.split(separator: " ") }
	var found: [Int] = []

	while pos < lines.count {
		let instruction = instructions[pos]

		if !found.contains(pos) {
			found.append(pos)
		} else {
            has_break = true
			break
		}

		switch instruction[0] {
		case "acc":
			acc += Int(instruction[1])!
			pos += 1
		case "jmp":
            if nop_jmp == nop_jmp_to_change {
                pos += 1
            } else {
                pos += Int(instruction[1])!
            }
            nop_jmp += 1
        case "nop":
            if nop_jmp == nop_jmp_to_change {
                pos += Int(instruction[1])!
            } else {
                pos += 1
            }
            nop_jmp += 1
		default:
			pos += 1
		}
	}

	return (acc, has_break)
}

main()

func getInputValue(name: String) -> String {
    switch name {
    case "example":
        return """
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"""
    case "input":
        return """
acc +0
acc -11
acc -19
acc -18
jmp +356
nop +29
acc +22
jmp +619
jmp +97
acc -14
jmp +71
nop +54
nop +348
jmp +144
jmp +123
nop +27
acc +48
acc +3
acc +2
jmp +79
jmp +576
acc -7
acc +37
acc +37
jmp +478
acc +49
nop +600
acc +28
jmp +388
acc +6
jmp +232
acc -2
jmp +1
jmp +140
acc +36
acc -15
acc +21
acc +29
jmp +361
acc +4
acc -2
jmp +585
acc +44
acc -16
acc +42
acc -12
jmp +252
acc +0
acc +19
acc +7
acc +38
jmp +232
acc +14
acc +26
jmp +132
acc +46
acc -19
jmp +219
acc +45
acc -1
acc +23
jmp +376
acc +41
nop +2
jmp +55
acc +24
acc +43
acc -3
jmp -53
acc +2
acc +19
jmp +520
jmp +1
acc +9
acc -2
acc +40
jmp +284
acc +0
jmp +250
jmp +454
jmp +559
acc +22
nop +561
jmp +425
jmp +146
jmp +236
jmp +1
jmp +222
acc -19
acc -10
acc -8
jmp +399
nop +410
acc +49
acc -1
jmp +209
jmp -88
jmp +263
acc +0
acc -5
acc +31
jmp +438
acc -10
acc +37
jmp +404
acc +34
acc +49
acc +0
jmp -79
acc +23
acc +0
nop -23
acc +47
jmp -71
jmp +277
acc +37
nop +144
acc +16
jmp +248
nop +179
jmp +413
jmp +177
acc +18
acc +1
acc +36
acc -19
jmp +145
acc -8
acc +34
jmp +154
nop +174
acc +14
acc +17
jmp +272
acc +46
acc +17
jmp +219
acc -3
acc +1
nop -81
jmp +335
acc -4
acc -18
acc +11
jmp +139
nop +221
jmp +1
acc +1
jmp +295
acc -4
jmp +205
jmp +270
nop +87
acc +11
acc +37
nop +61
jmp +319
acc +39
acc -16
jmp +327
acc +34
acc -14
acc -5
nop -142
jmp +417
jmp +1
nop +467
acc +7
jmp -87
nop +186
nop +158
acc +44
jmp -157
jmp +281
acc +14
acc +29
acc +40
jmp +132
jmp +1
jmp +1
acc -18
jmp +345
jmp +1
acc +0
acc +49
jmp +9
acc +30
jmp +1
jmp +261
acc +38
acc +42
acc -6
nop +369
jmp +256
nop -173
jmp +6
acc +44
acc -4
acc +46
acc -2
jmp +229
jmp +1
acc +44
acc +26
jmp -74
acc +25
nop -55
acc -15
acc +24
jmp -90
acc +15
jmp +214
acc +48
nop -35
acc -1
jmp +287
jmp +249
acc -11
acc +36
nop +407
nop -75
jmp +405
acc -14
acc +9
acc -14
jmp -204
acc +34
acc +31
nop +171
jmp +63
acc +7
acc +1
acc +23
jmp -222
acc +17
acc -13
acc +13
acc -6
jmp +401
acc +15
acc -10
acc +38
jmp -146
acc +9
nop +155
jmp -211
acc -14
acc +41
nop +163
acc -16
jmp +54
acc +3
acc +1
jmp -108
acc +42
nop -77
acc -6
jmp -27
acc +12
jmp +231
jmp +321
jmp -39
acc +16
acc +41
acc +1
jmp -114
acc +10
acc -2
acc -18
acc +7
jmp +220
jmp +103
nop +144
acc +39
nop -186
jmp +85
acc -17
acc +5
acc +36
acc -14
jmp +369
acc +3
jmp +101
jmp +38
acc +16
acc +16
acc +49
acc +35
jmp +169
jmp +190
jmp +1
jmp -226
acc +15
jmp -83
acc -2
acc -1
acc +11
jmp -175
nop +305
acc +12
acc +34
nop +153
jmp +294
jmp -189
acc +8
jmp +334
acc +23
acc +48
jmp +146
jmp -63
nop +329
acc +25
nop -3
acc +4
jmp -209
acc +39
acc +30
acc +22
acc +35
jmp +292
jmp +29
acc +14
acc +48
acc -2
jmp +92
acc +25
acc +3
jmp +72
nop +180
acc +7
jmp +1
acc +18
jmp -159
jmp +181
acc +15
jmp -46
acc -7
acc +46
acc +25
jmp +252
acc -2
acc +50
acc +24
acc -2
jmp -272
acc +20
acc +38
acc -17
jmp +12
acc -2
jmp +136
acc +14
acc +32
acc +50
jmp -83
acc +35
acc -10
jmp -118
acc +4
jmp -325
jmp +136
acc -5
nop +164
acc -8
acc -8
jmp +174
jmp -38
jmp +108
nop -8
acc +8
jmp +196
nop -234
acc +47
jmp +260
acc +31
acc +26
acc -8
jmp +96
acc +0
nop -294
acc +3
acc +0
jmp -330
nop +1
acc +32
acc +36
jmp +160
nop -201
acc +24
acc +48
jmp -114
acc +32
nop +251
jmp +233
acc +22
nop -330
acc +8
jmp +1
jmp +67
nop +115
nop -304
jmp +171
acc +2
acc +7
jmp -55
nop +186
jmp +214
acc +12
nop -148
nop -388
jmp -232
acc -11
acc +1
jmp -98
acc +39
jmp -250
jmp -337
nop -388
acc +49
acc +45
jmp -54
acc +17
acc -8
jmp -57
jmp +209
jmp -231
jmp +1
jmp +1
jmp -124
acc +49
acc +17
jmp +1
acc +45
jmp -84
acc +3
acc -3
jmp -402
jmp +1
acc -8
acc -7
acc +17
jmp -30
jmp +54
acc +2
jmp +1
jmp +75
nop -224
acc +16
jmp -270
acc +43
acc +34
jmp -68
acc +45
jmp -4
acc +23
jmp -421
jmp -152
acc +47
acc -19
jmp -361
jmp -259
acc +20
acc +0
jmp -187
jmp -188
nop +10
nop -368
acc -5
jmp -403
acc +45
acc -12
nop -357
jmp -51
jmp -139
jmp -258
nop -464
acc +49
jmp +37
jmp -359
acc +30
jmp -315
acc -9
acc +5
acc +28
acc +5
jmp -187
acc -9
acc +47
jmp -133
jmp +50
acc +19
acc +8
jmp -81
acc -3
acc +18
jmp -265
nop -53
jmp +1
jmp -164
acc +44
nop -322
jmp -76
acc -17
acc +42
acc +8
acc +2
jmp -421
jmp -285
acc +41
acc -2
jmp +133
acc +13
nop -47
jmp -340
acc +40
acc -16
jmp +1
acc +13
jmp +115
jmp +77
acc -10
acc -9
acc -16
acc +17
jmp -264
jmp -126
acc +49
jmp -98
acc +26
acc -19
acc +24
acc +34
jmp -338
acc +13
jmp -242
acc +7
acc -5
nop -233
jmp -234
acc -12
acc +4
jmp +62
acc +9
nop -485
acc +9
jmp -236
jmp +1
acc -16
acc +20
nop -497
jmp +11
acc +41
acc +8
acc +0
acc +49
jmp -172
acc +0
acc +0
acc +23
jmp -501
jmp -495
nop -285
acc +22
acc +36
jmp -103
jmp -513
acc +0
acc +0
jmp -480
nop -254
acc +31
jmp -96
acc +9
acc +18
acc +27
acc +0
jmp -431
acc +34
acc +31
nop -104
jmp -66
acc -5
acc +30
acc +21
nop -362
jmp -471
acc +7
acc +14
acc +47
nop -184
jmp -561
acc -1
jmp -36
acc +42
acc +17
jmp -306
acc +3
acc -11
acc +15
acc +40
jmp -481
acc +30
jmp -537
acc +45
nop -358
jmp -322
nop -169
nop -298
acc +14
acc +0
jmp +23
acc -14
acc +43
nop -111
jmp -492
acc +43
acc +19
acc +44
acc +9
jmp -365
acc +25
acc +24
acc +5
acc +0
jmp -256
jmp -488
acc +17
jmp -170
nop -17
acc +50
acc +5
nop -494
jmp -292
jmp -234
acc +42
acc -1
nop -365
acc -15
jmp -47
jmp +1
acc -9
jmp -286
jmp -523
acc -13
acc +1
acc +27
acc +0
jmp -393
jmp -327
acc -4
acc +37
nop -375
acc +38
jmp +1
"""
    default:
        return ""
    }
}
