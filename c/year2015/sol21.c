#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#define LEN(array) (int)((sizeof(array)) / (sizeof(array[0])))

typedef struct {
  int hitpoints;
  int damage;
  int armor;
} player_t;

typedef struct {
  char name[16];
  int cost;
  int damage;
  int armor;
} item_t;

// You must buy exactly one weapon.
static item_t weapons[] = {
    {.name = "Dagger", .cost = 8, .damage = 4, .armor = 0},
    {.name = "Shortsword", .cost = 10, .damage = 5, .armor = 0},
    {.name = "Warhammer", .cost = 25, .damage = 6, .armor = 0},
    {.name = "Longsword", .cost = 40, .damage = 7, .armor = 0},
    {.name = "Greataxe", .cost = 74, .damage = 8, .armor = 0},
};

// Armor is optional, but you can't use more than one.
static item_t armors[] = {
    {.name = "", .cost = 0, .damage = 0, .armor = 0},  // noop
    {.name = "Leather", .cost = 13, .damage = 0, .armor = 1},
    {.name = "Chainmail", .cost = 31, .damage = 0, .armor = 2},
    {.name = "Splintmail", .cost = 53, .damage = 0, .armor = 3},
    {.name = "Bandedmail", .cost = 75, .damage = 0, .armor = 4},
    {.name = "Platemail", .cost = 102, .damage = 0, .armor = 5},
};

// You can buy 0-2 unique rings (at most one for each hand).
static item_t rings[] = {
    {.name = "", .cost = 0, .damage = 0, .armor = 0},  // noop
    {.name = "", .cost = 0, .damage = 0, .armor = 0},  // noop
    {.name = "Damage +1", .cost = 25, .damage = 1, .armor = 0},
    {.name = "Damage +2", .cost = 50, .damage = 2, .armor = 0},
    {.name = "Damage +3", .cost = 100, .damage = 3, .armor = 0},
    {.name = "Defense +1", .cost = 20, .damage = 0, .armor = 1},
    {.name = "Defense +2", .cost = 40, .damage = 0, .armor = 2},
    {.name = "Defense +3", .cost = 80, .damage = 0, .armor = 3},
};

// Return true if the player wins, false otherwise.
static bool player_win(player_t player, player_t boss) {
  for (int turn = 0; player.hitpoints > 0 && boss.hitpoints > 0; turn++) {
    if (turn % 2 == 0) {  // player's turn
      int damage = player.damage - boss.armor;
      boss.hitpoints -= damage > 0 ? damage : 1;
    } else {  // boss's turn
      int damage = boss.damage - player.armor;
      player.hitpoints -= damage > 0 ? damage : 1;
    }
  }
  return player.hitpoints > 0;
}

int year2015_sol21(void) {
  player_t boss = {.hitpoints = 100, .damage = 8, .armor = 2};
  player_t player = {.hitpoints = 100, .damage = 0, .armor = 0};

  int mincost = 0;
  int maxcost = 0;
  for (int w = 0; w < LEN(weapons); w++) {
    for (int a = 0; a < LEN(armors); a++) {
      for (int r1 = 0; r1 < LEN(rings); r1++) {
        for (int r2 = 0; r2 < LEN(rings); r2++) {
          if (r1 == r2) {  // both rings cannot be the same
            continue;
          }
          int cost = weapons[w].cost + armors[a].cost + rings[r1].cost +
                     rings[r2].cost;
          // If both the mincost and maxcost has some initial value
          // which is not 0 and the current cost is both greater than or
          // equal to the mincost and smaller than or equal to the
          // maxcost, continue.
          if (mincost && maxcost && cost >= mincost && cost <= maxcost) {
            continue;
          }

          player.armor = weapons[w].armor + armors[a].armor + rings[r1].armor +
                         rings[r2].armor;
          player.damage = weapons[w].damage + armors[a].damage +
                          rings[r1].damage + rings[r2].damage;

          bool outcome = player_win(player, boss);  // true if player wins
          if (outcome && (!mincost || cost < mincost)) {
            mincost = cost;
          } else if (!outcome && cost > maxcost) {
            maxcost = cost;
          }
        }
      }
    }
  }

  printf("21.1: %d\n21.2: %d\n", mincost, maxcost);
  return EXIT_SUCCESS;
}
