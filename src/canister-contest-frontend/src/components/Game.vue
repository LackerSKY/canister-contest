<template>
  <div class="p-6 max-w-md mx-auto bg-white rounded-xl shadow-lg space-y-6">
    <h2 class="text-2xl font-bold text-center mb-4 text-gray-800">Tic-Tac-Toe</h2>
    <div class="flex justify-between mb-4">
      <div class="text-lg font-semibold text-gray-700">
        Player 1: <span class="font-normal">{{ p1 }} (X)</span>
      </div>
      <div class="text-lg font-semibold text-gray-700">
        Player 2: <span class="font-normal">{{ p2 }} (O)</span>
      </div>
    </div>
    <div class="grid grid-cols-3 gap-2 mx-auto w-64">
      <div
        v-for="(cell, index) in board"
        :key="index"
        class="flex items-center justify-center h-24 w-24 bg-gray-100 text-4xl font-bold text-gray-700 border border-gray-300 rounded-md cursor-pointer transition-colors duration-300 ease-in-out hover:bg-gray-200 active:bg-gray-300"
        @click="makeMove(index)"
      >
        {{ cell }}
      </div>
    </div>
    <div v-if="winner" class="mt-4 text-xl font-semibold text-green-600 text-center">
      <h2>{{ winnerMessage }}</h2>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { defineProps } from 'vue';

const props = defineProps({
  p1: String,
  p2: String
});

const board = ref(Array(9).fill(''));
const currentPlayer = ref('X');
const winner = ref(null);

const makeMove = (index) => {
  if (!board.value[index] && !winner.value) {
    board.value[index] = currentPlayer.value;
    if (checkWinner()) {
      winner.value = currentPlayer.value;
    } else {
      currentPlayer.value = currentPlayer.value === 'X' ? 'O' : 'X';
    }
  }
};

const checkWinner = () => {
  const winPatterns = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6]
  ];

  return winPatterns.some(pattern => {
    const [a, b, c] = pattern;
    return board.value[a] && board.value[a] === board.value[b] && board.value[a] === board.value[c];
  });
};

const winnerMessage = computed(() => {
  if (winner.value) {
    return `Winner is ${winner.value === 'X' ? props.p1 : props.p2}!`;
  }
  return '';
});
</script>