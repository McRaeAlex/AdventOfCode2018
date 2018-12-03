import sys


class layout():
    def __init__ (self, id, x, y, width, height):
        self.id = id
        self.x = x
        self.y = y
        self.width = width
        self.height = height
    
    def __repr__ (self):
        return "id: %d, xy: (%d, %d), width: %d, height: %d" % (self.id, self.x, self.y, self.width, self.height)

def checkOverLap(item1, item2):
    area1 = (item1.width) * (item1.height)
    area2 = (item2.width) * (item2.height)
    areaI = (min(item1.x + item1.width, item2.x + item2.width)\
            -  max(item1.x, item2.x)) *\
            (min(item1.y + item1.height, item2.y + item2.height)\
            - max(item1.y, item2.y))
    return (areaI)

def part1():
    # read the file into a list of structs
    #lines = open('/home/alex/Coding/AdventOfCode2018/day3/test1.txt', 'r')
    lines = sys.stdin.readlines()
    listOfLayout = []
    for line in lines:
        tmparr1 = line.split(' ')
        id = int(tmparr1[0][1:])
        xy = tmparr1[2].split(',') # get the xy
        x = int(xy[0])
        y = int(xy[1][:-1]) # remove the last character
        wh = tmparr1[3].split('x')
        w = int(wh[0]) # get w
        h = int(wh[1][:-1])
        listOfLayout.append(layout(id, x, y, w, h))

    # compare each struct and find the overlaying data
    overlay = 0
    arry = zip([i for i in range(len(listOfLayout))], listOfLayout)
    for (i, item1) in arry:
        for item2 in listOfLayout[i:]:
            if item1.id == item2.id:
                continue
            overlay += checkOverLap(item1, item2)
    print(overlay)

if __name__ == "__main__":
    part1()