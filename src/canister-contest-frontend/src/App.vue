<template>
  <main>
    <Login @login="handleLogin" v-if="hideComponent" />
    <Game v-if="showComponent" :p1="p1" :p2="p2" :wins="wins" @logout="handleLogout" @updateWins="handleUpdateWins" />
    <div class="flex w-full m-4">
    <button
        @click="showLeaderboard = !showLeaderboard"
        class="py-2 px-4 bg-blue-600 text-white font-semibold rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 mx-auto"
      >{{ showLeaderboard ? 'Hide Leaderboard' : 'Show Leaderboard' }}
    </button>
    </div>
    <Leaderboard v-if="showLeaderboard" />
    
    <br />
    <br />
    <img src="/logo2.svg" alt="DFINITY logo" />
  </main>
</template>

<script setup>
import { ref } from 'vue';
import Login from "./components/Login.vue";
import Game from "./components/Game.vue";
import Leaderboard from "./components/Leaderboard.vue";

const showComponent = ref(false);
const hideComponent = ref(true);
const showLeaderboard = ref(false);

const p1 = ref('');
const p2 = ref('');
const wins = ref({ p1: 0, p2: 0 }); 

const handleLogin = (player1, player2) => {
  p1.value = player1;
  p2.value = player2;
  showComponent.value = true;
  hideComponent.value = false;
};

const handleLogout = () => {
  showComponent.value = false;
  hideComponent.value = true;
};

const handleUpdateWins = (newWins) => {
  wins.value = newWins;
};
</script>
