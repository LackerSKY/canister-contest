<template>
  <div class="p-8 max-w-4xl mx-auto bg-white rounded-lg shadow-lg space-y-6">
    <h2 class="text-3xl font-bold text-center text-gray-800">Tic-Tac-Toe</h2>
    <div class="flex flex-col space-y-4 mb-4">
      <div class="flex justify-between text-lg font-semibold text-gray-700">
        <div>
          <span>Player 1: </span>
          <span class="font-normal">{{ p1 }} (X)</span>
        </div>
        <div>
          <span>Player 2: </span>
          <span class="font-normal">{{ p2 }} (O)</span>
        </div>
      </div>
      <div class="flex justify-between text-lg font-semibold text-gray-700">
        <div class="text-center">
          <span class="font-normal">Wins: {{ wins.p1 }}</span>
        </div>
        <div class="text-center">
          <span class="font-normal">Wins: {{ wins.p2 }}</span>
        </div>
      </div>
    </div>
    <div class="grid grid-cols-3 gap-4 mx-auto max-w-[400px]">
      <div
        v-for="(cell, index) in board"
        :key="index"
        class="flex items-center justify-center h-32 w-32 bg-gray-200 text-5xl font-bold text-gray-700 border border-gray-300 rounded-md cursor-pointer transition-transform duration-300 ease-in-out hover:bg-gray-300 active:bg-gray-400"
        @click="makeMove(index)"
      >
        {{ cell }}
      </div>
    </div>
    <div v-if="winner" class="mt-4 text-xl font-semibold text-green-600 text-center">
      <h2>{{ winnerMessage }}</h2>
    </div>
    <div class="flex space-x-4 mt-6">
      <button
        @click="resetBoard"
        class="w-1/2 py-2 bg-red-600 text-white font-semibold rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500"
      >Reset Board
      </button>
      <button
        @click="goToLogin"
        class="w-1/2 py-2 bg-gray-600 text-white font-semibold rounded-md hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500"
      >Back to Login
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { defineProps } from 'vue';

const props = defineProps({
  p1: String,
  p2: String,
  onLogout: Function
});

const board = ref(Array(9).fill(''));
const currentPlayer = ref('X');
const winner = ref(null);
const wins = ref({ p1: 0, p2: 0 });

const makeMove = (index) => {
  if (!board.value[index] && !winner.value) {
    board.value[index] = currentPlayer.value;
    if (checkWinner()) {
      winner.value = currentPlayer.value;
      if (currentPlayer.value === 'X') {
        wins.value.p1++;
      } else {
        wins.value.p2++;
      }
    } else {
      currentPlayer.value = currentPlayer.value === 'X' ? 'O' : 'X';
    }
  }
};

const resetBoard = () => {
  board.value = Array(9).fill('');
  currentPlayer.value = 'X';
  winner.value = null;
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

const handleKeydown = (event) => {
  const key = event.key;
  let index;

  switch (key) {
    case '7':
      index = 0;
      break;
    case '8':
      index = 1;
      break;
    case '9':
      index = 2;
      break;
    case '4':
      index = 3;
      break;
    case '5':
      index = 4;
      break;
    case '6':
      index = 5;
      break;
    case '1':
      index = 6;
      break;
    case '2':
      index = 7;
      break;
    case '3':
      index = 8;
      break;
    case 'Escape':
      resetBoard();
      return; 
    default:
      return; 
  }

  if (index !== undefined) {
    makeMove(index);
  }
};

const goToLogin = () => {
  props.onLogout(); 
};

onMounted(() => {
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>