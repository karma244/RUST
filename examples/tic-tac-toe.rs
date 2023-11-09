import random

def istaked(state, x):
    if state[x-1] == 'X' or state[x-1] == 'O':
        return True;
    return False;

def update_field(state):
    print("----STATE----");
    print(" %c | %c | %c " % (state[0], state[1], state[2]))
    print("___|___|___")
    print(" %c | %c | %c " % (state[3], state[4], state[5]))
    print("___|___|___")
    print(" %c | %c | %c " % (state[6], state[7], state[8]))
    print("-------------\n");

def turn_changed(state, player, flag):
    while True:
        if flag == False:
            x = int(input("Player: {}\n숫자를 입력하세요.\n".format(player)))
            if istaked(state, x):
                print("이미 선택된 칸입니다.")
            else:
                state[x-1] = player
                return state
        else:
            if player == 'O':
                x = int(input("당신의 차례입니다.\n숫자를 입력하세요.\n".format(player)))
                if istaked(state, x):
                    print("이미 선택된 칸입니다.")
                else:    
                    state[x-1] = player
                    return state
            ran = random.randint(0, 9)
            print(ran)
            if istaked(state, ran):
                print("이미 선택된 칸입니다.")
            else:
                state[ran-1] = player
                return state
            
def exist_winner(state):
    for i in range(0, 3):
        if state[i] == state[i + 3] and state[i] == state[i + 6]:
            return True;
        
    for i in range(0, 7, 3):
        if state[i] == state[i + 1] and state[i] == state[i + 2]:
            return True;

    if (state[0] == state[4] and state[0] == state[8]) or (state[2] == state[4] and state[2] == state[6]):
        return True;

    return False;

def check_draw(state):
    return all(x == 'O' or x == 'X' for x in state)

state = ['1', '2', '3', '4', '5', '6', '7', '8', '9']
now_player = 'X'
flag = input("AI와 대련 하시겠습니까? (무적 아님) O/X: ")

while True:
    update_field(state)
    
    if flag == 'O':
        state = turn_changed(state, now_player, True)
    else:
        state = turn_changed(state, now_player, False)
        
    if exist_winner(state):
        update_field(state)
        print("{}이/가 이겼습니다".format(now_player))
        break
    
    if check_draw(state):
        print("비겼습니다. 게임이 종료됩니다.")
        break
    
    if now_player == 'X': now_player = 'O'
    else: now_player = 'X'
