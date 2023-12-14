symbolPositionsByLine = {}
numbersByLine = {}

with open('../input.txt', 'r') as f:
    lines = f.readlines()
    f.close()
    
    numberBuffer = None
    numberStart = None
    numberLen = None

    for lineidx, line in enumerate(lines):
        for charidx, char in enumerate(line.strip()):
            if(char.isnumeric()):
                if numberBuffer is None:
                    numberBuffer = char
                    numberStart = charidx
                    numberLen = 1
                else:
                    numberBuffer += char
                    numberLen += 1
            else:
                if numberBuffer is not None:
                    if lineidx not in numbersByLine:
                        numbersByLine[lineidx] = []
                    numbersByLine[lineidx].append((numberStart, numberLen, numberBuffer))
                    numberBuffer = None
                    numberLen = None
                    numberStart = None
                if(char != '.' and not(char.isnumeric())):
                    if lineidx not in symbolPositionsByLine:
                        symbolPositionsByLine[lineidx] = []
                    symbolPositionsByLine[lineidx].append((char,charidx))

        if numberBuffer is not None:
            if lineidx not in numbersByLine:
                numbersByLine[lineidx] = []
            numbersByLine[lineidx].append((numberStart, numberLen, numberBuffer))
            numberBuffer = None
            numberLen = None
            numberStart = None

validSum = 0

for line in numbersByLine:
    for numberTuple in numbersByLine[line]:
        numberStart = numberTuple[0]
        numberLen = numberTuple[1]
        number = int(numberTuple[2])
        numberAdded = False

        #if line number greater than 0, get all symbols in previous line at positions between numberStart - 1 and numberStart + numberLen + 1
        if (line - 1) in symbolPositionsByLine.keys():
            #if there is a symbol in the previous line at positions between numberStart - 1 and numberStart + numberLen + 1, add number to validSum
            for symbol in symbolPositionsByLine[line - 1]:
                if not numberAdded and symbol[1] >= numberStart - 1 and symbol[1] <= numberStart + numberLen:
                    validSum += number
                    numberAdded = True
            if numberAdded:
                continue

       #if line is not last line, get all symbols in next line at positions between numberStart - 1 and numberStart + numberLen + 1
       # if there is a symbol in the next line at positions between numberStart - 1 and numberStart + numberLen + 1, add number to validSum
        if (line + 1) in symbolPositionsByLine.keys():
            for symbol in symbolPositionsByLine[line + 1]:
                if not numberAdded and symbol[1] >= numberStart - 1 and symbol[1] <= numberStart + numberLen:
                    validSum += number
                    numberAdded = True
            if numberAdded:
                continue
        
        #check current line for symbols at positions numberStart - 1 or numberStart + numberLen + 1
        if line in symbolPositionsByLine.keys():
            for symbol in symbolPositionsByLine[line]:
                if not numberAdded and symbol[1] == numberStart - 1 or symbol[1] == numberStart + numberLen:
                    validSum += number
                    numberAdded = True
            if numberAdded:
                continue

print(validSum) 