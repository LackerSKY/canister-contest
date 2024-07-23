<template>
    <div class="p-6 max-w-md mx-auto bg-white rounded-xl shadow-md space-y-4">
      <h2 class="text-2xl font-bold text-center mb-4">Tic-Tac-Toe Game!</h2>
      <div class="space-y-4">
        <div class="flex flex-col">
          <label for="player1" class="text-lg font-semibold">Enter player 1 name (X)</label>
          <input
            id="player1"
            type="text"
            v-model="p1"
            class="mt-2 p-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div class="flex flex-col">
          <label for="player2" class="text-lg font-semibold">Enter player 2 name (O)</label>
          <input
            id="player2"
            type="text"
            v-model="p2"
            class="mt-2 p-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
      </div>
      <button
        @click="startGame"
        class="w-full py-2 bg-blue-600 text-white font-semibold rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
      >Start Game
      </button>
    </div>
  </template>
  
  <script setup>
  import { ref } from 'vue';
  import { defineEmits } from 'vue';
  
  const emit = defineEmits(['login']);
  
  const p1 = ref('');
  const p2 = ref('');
  
  const startGame = () => {
    if (p1.value && p2.value) {
      emit('login', p1.value, p2.value);
    } else {
      alert('Please enter names for both players.');
    }
  };
  </script>