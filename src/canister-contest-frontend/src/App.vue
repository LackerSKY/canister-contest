<script setup>
import { ref } from 'vue';
import Login from "./components/Login.vue";
import Game from "./components/Game.vue";

const showComponent = ref(false);
const hideComponent = ref(true);

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

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <Login @login="handleLogin" v-if="hideComponent" />
    <Game v-if="showComponent" :p1="p1" :p2="p2" :wins="wins" @logout="handleLogout" @updateWins="handleUpdateWins" />
  </main>
</template>
