"""flame.py

Flames your item specified number of times, output is best flame and number of times threshold is met.

"""
import logging
import concurrent.futures
import argparse
import random
import numpy
from numpy.random import choice
import time
from tqdm import tqdm

"""Vec<&str>"""
options = ['str', 'dex', 'int', 'luk', 'strdex', 'strint', 'strluk', 'dexint', 'dexluk', 'intluk', 'hp', 'mp', 'lvlred', 'def', 'att', 'matt', 'speed', 'jmp', 'as']

"""Vec<(&str, Vec<f32>)>"""
tier_table = {'drop' : [.25, .3, .3, .14, .01], 
              'pflame' : [.2, .3, .36, .14, 0],
              'eflame' : [0, .29, .45, .25, .01], 
              'regcraft': [.5, .4, .1, 0, 0],
              'mastercraft': [.15, .3, .4, .14, .01],
              'meistercraft': [0, .19, .5, .3, .01],
              'masterfuse': [.25, .35, .30, .1, 0],
              'meisterfuse': [0, .4, .45, .14, .01]}

"""Vec<(&str, Vec<u32>)>"""
flat_options = {'100-109': [6, 12, 18, 24, 30, 36, 42],
                '110-119': [6, 12, 18, 24, 30, 36, 42],
                '120-129': [7, 14, 21, 28, 35, 42, 49],
                '130-139': [7, 14, 21, 28, 35, 42, 49],
                '140-149': [8, 16, 24, 32, 40, 48, 56],
                '150-159': [8, 16, 24, 32, 40, 48, 56],
                '160-169': [9, 18, 27, 36, 45, 54, 63],
                '170-179': [9, 18, 27, 36, 45, 54, 63],
                '180-189': [10, 20, 30, 40, 50, 60, 70],
                '190-199': [10, 20, 30, 40, 50, 60, 70],
                '200-249': [11, 22, 33, 44, 55, 66, 77],
                '250+': [12, 24, 36, 48, 60, 72, 84]}

"""Vec<(&str, Vec<u32>)>"""
combo_options = {'100-109': [3, 6, 9, 12, 15, 18, 21],
                 '110-119': [3, 6, 9, 12, 15, 18, 21],
                 '120-129': [4, 8, 12, 16, 20, 24, 28],
                 '130-139': [4, 8, 12, 16, 20, 24, 28],
                 '140-149': [4, 8, 12, 16, 20, 24, 28],
                 '150-159': [4, 8, 12, 16, 20, 24, 28],
                 '160-169': [5, 10, 15, 20, 25, 30, 35],
                 '170-179': [5, 10, 15, 20, 25, 30, 35],
                 '180-189': [5, 10, 15, 20, 25, 30, 35],
                 '190-199': [5, 10, 15, 20, 25, 30, 35],
                 '200-249': [6, 12, 18, 24, 30, 36, 42],
                 '250+': [7, 14, 21, 28, 35, 42, 49]}

"""Vec<(&str, Vec<u32>)>"""
hp_mp_options = {'100-109': [300, 600, 900, 1200, 1500, 1800, 2100],
                 '110-119': [330, 660, 990, 1320, 1650, 1980, 2310],
                 '120-129': [360, 720, 1080, 1440, 1800, 2160, 2520],
                 '130-139': [390, 780, 1170, 1560, 1950, 2340, 2730],
                 '140-149': [420, 840, 1260, 1680, 2100, 2520, 2940],
                 '150-159': [450, 900, 1350, 1800, 2250, 2700, 3150],
                 '160-169': [480, 960, 1440, 1920, 2400, 2880, 3360],
                 '170-179': [510, 1020, 1530, 2040, 2550, 3060, 3570],
                 '180-189': [540, 1080, 1620, 2160, 2700, 3240, 3780],
                 '190-199': [570, 1140, 1710, 2280, 2850, 3420, 3990],
                 '200-249': [600, 1200, 1800, 2400, 3000, 3600, 4200],
                 '250+': [700, 1400, 2100, 2800, 3500, 4200, 4900]}

"""Vec<(&str, Vec<u32>)>"""
basic_tiers = [1, 2, 3, 4, 5, 6, 7]

"""Vec<&str, arr[(&str, arr[7, u32])]>"""
basic_options = {'100-109': basic_tiers,
                 '110-119': basic_tiers,
                 '120-129': basic_tiers,
                 '130-139': basic_tiers,
                 '140-149': basic_tiers,
                 '150-159': basic_tiers,
                 '160-169': basic_tiers,
                 '170-179': basic_tiers,
                 '180-189': basic_tiers,
                 '190-199': basic_tiers,
                 '200-249': basic_tiers,
                 '250+': basic_tiers}

"""Vec<(&str, Vec<(&str, Vec<u32>)>)>"""
option_table = {'str': flat_options, 'dex': flat_options, 'int': flat_options, 'luk': flat_options, 
                'strdex': combo_options, 'strint': combo_options, 'strluk': combo_options, 'dexint': combo_options, 'dexluk': combo_options, 'intluk': combo_options,
                'hp': hp_mp_options, 'mp': hp_mp_options, 'lvlred': basic_options, 'def': basic_options, 
                'att': basic_options, 'matt': basic_options, 'speed': basic_options, 'jmp': basic_options, 'as': basic_options}

def select_option(roll, used_options):
    used_options.sort()
    for used_option in used_options:
        if roll >= used_option:
            roll += 1
        else:
            return roll
    return roll

def roll_tier(option_method):
    roll = random.random()
    sum = 0
    for tier, threshold in reversed(list(enumerate(tier_table[option_method]))):
        sum += threshold
        if roll >= 1.0 - sum:
            return tier
    return 0
    
def evaluate_flame(flame_lines):
    stats = {'str': 0, 'dex': 0, 'int': 0, 'luk': 0, 'hp': 0, 'mp': 0, 'att': 0, 'matt': 0, 'as': 0}
    for flame_line in flame_lines:
        if flame_line[0] in ['str', 'dex', 'int', 'luk', 'att', 'matt', 'hp', 'mp', 'as']:
            stats[flame_line[0]] += flame_line[1]
        elif flame_line[0] == 'strdex':
            stats['str'] += flame_line[1]
            stats['dex'] += flame_line[1]
        elif flame_line[0] == 'strint':
            stats['str'] += flame_line[1]
            stats['int'] += flame_line[1]
        elif flame_line[0] == 'strluk':
            stats['str'] += flame_line[1]
            stats['luk'] += flame_line[1]
        elif flame_line[0] == 'dexint':
            stats['dex'] += flame_line[1]
            stats['int'] += flame_line[1]
        elif flame_line[0] == 'dexluk':
            stats['dex'] += flame_line[1]
            stats['luk'] += flame_line[1]
        elif flame_line[0] == 'intluk':
            stats['luk'] += flame_line[1]
            stats['int'] += flame_line[1]
    return stats
    
def score_flame(stats, type):
    all_stat = 9.9
    sub_stat = 0.083
    att = 2.96
    all_stat_x = 20
    att_x = 6
    hpmp = 110
    all_stat_d = 10
    att_d = 20
    if type == 'str' or type == 'luk':
        return stats['as'] * all_stat + stats['att'] * att + stats[type] + stats['dex'] * sub_stat
    elif type == 'dex':
        return stats['as'] * all_stat + stats['att'] * att + stats[type] + stats['str'] * sub_stat
    elif type == 'int':
        return stats['as'] * all_stat + stats['matt'] * att + stats[type] + stats['luk'] * sub_stat
    elif type == 'kanna':
        return stats['as'] * all_stat + stats['matt'] * att + stats['int'] + stats['luk'] * sub_stat + stats['hp'] / hpmp + stats['mp'] / hpmp
    elif type == 'hp':
        return stats['as'] * all_stat_d + stats['att'] * att_d + stats[type] + stats['str'] * sub_stat
    elif type == 'alt_thief':
        return stats['as'] * all_stat + stats['att'] * att + stats['luk'] + stats['str'] * sub_stat +  stats['dex'] * sub_stat
    elif type == 'as':
        return stats['as'] * all_stat_x + stats['att'] * att_x + stats['dex'] + stats['str'] + stats['luk']

def main():
    with concurrent.futures.ProcessPoolExecutor() as executor:
        parser = argparse.ArgumentParser(usage=__doc__)
        parser.add_argument('-type', type=str, default='pflame', help='options are: pflame, eflame, regcraft, mastercraft, meistercraft, masterfuse, meisterfuse')
        parser.add_argument('-noboss', action='store_true', help="include this argument with no value to indicate not boss flame")
        parser.add_argument('-trials', type=int, default=100000, help="number of flames to use")
        parser.add_argument('-level', type=str, default='140-149', help='equip level range, options are: 100-109, 110-119, 120-129, 130-139, 140-149, 150-159, 160-169, 170-179, 180-189, 190-199, 200-249, 250+')
        parser.add_argument('-threshold', type=int, default=110, help='flame score to keep (for statistics)')
        parser.add_argument('-stat', type=str, default='str', help='specify desired stat str, dex, int, luk, as, kanna, da, alt_thief (default str)')
        args = parser.parse_args()

        boss_flame = not args.noboss
        equip_lvl = args.level
        flames = []
        keep = 0
        max_flame = 0
        max_flame_lines = []
        start = time.time()
        for i in tqdm(range(args.trials)): 
            if args.noboss:
                # https://docs.google.com/spreadsheets/d/14bKXNRYgC7Xa9S18b0jSi6tY6udOHH5B-S4PZtysN_4/
                # OLD DATA --- 67/202, 93/202, 37/202, 5/202
                #
                # NEW DATA FROM ABOVE SPREADSHEET
                # 144/367, 144/367, 67/367, 12/367
                #
                # STELLA DATA (stella_data.txt)
                # 122/315, 120/315, 58/315, 15/315
                #
                # AGGREGATE DATA WEIGHTS = 0.39, 0.39, 0.18, 0.04

                nonboss_lines = [1, 2, 3, 4]
                nonboss_weighted_choice = random.choices(nonboss_lines, [0.39, 0.39, 0.18, 0.04], k=1)
            used_options = []
            for i in range(4 if boss_flame else nonboss_weighted_choice[0]):
                roll = select_option(random.randrange(19 - i), used_options)
                used_options.append(roll)
            flame_lines = []
            for used_option in used_options:
                option_id = options[used_option]
                tier = roll_tier(args.type) + (2 if boss_flame else 0)
                res = option_table[option_id][equip_lvl][tier]
                flame_lines.append([option_id, res])

            flame_stats = evaluate_flame(flame_lines)
            #print(flame_stats)
            score = score_flame(flame_stats, args.stat)
            flames.append(score)
            if score >= args.threshold:
                keep += 1
                if score > max_flame:
                    max_flame = score
                    max_flame_lines = flame_stats

        avg_flames = int(args.trials / keep)
        avg_cost = float(args.trials / keep) * 9120000 / 1000000000

        print()
        print(' settings: level: {}, threshold: {}, type: {}, noboss: {}'.format(args.level, args.threshold, args.type, args.noboss))
        print()
        print(' best score: {:0.3f}'.format(max_flame))
        print(' stats: {}'.format(max_flame_lines))
        print()
        print(' score above {}: {} out of {} trials'.format(args.threshold, keep, args.trials))
        print(' average flames: {:0d}'.format(avg_flames))
        if args.type == 'pflame':
            print(' average meso cost: {:0.3f}b'.format(avg_cost))
        print()
        print(' time: {:0.3f} seconds'.format(time.time() - start))
        print()

if __name__ == "__main__":
    main()