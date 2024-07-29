<template>
  <div class="p-4 sm:p-8 max-w-full md:max-w-4xl mx-auto bg-white rounded-lg shadow-lg space-y-4 sm:space-y-6">
    <h2 class="text-xl sm:text-2xl md:text-3xl font-bold text-center text-gray-800">Leaderboard</h2>
    <table class="w-full border-collapse text-xs sm:text-sm md:text-base">
      <thead>
        <tr>
          <th class="px-2 sm:px-4 py-1 sm:py-2 border-b text-left">Player 1</th>
          <th class="px-2 sm:px-4 py-1 sm:py-2 border-b text-left">Score</th>
          <th class="px-2 sm:px-4 py-1 sm:py-2 border-b text-left">Player 2</th>
          <th class="px-2 sm:px-4 py-1 sm:py-2 border-b text-left">Score</th>
          <th class="px-2 sm:px-4 py-1 sm:py-2 border-b text-left">Score difference</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(entry, index) in leaderboard" :key="index">
          <td class="px-2 sm:px-4 py-1 sm:py-2 border-b">{{ entry[0] }}</td>
          <td class="px-2 sm:px-4 py-1 sm:py-2 border-b">{{ entry[1] }}</td>
          <td class="px-2 sm:px-4 py-1 sm:py-2 border-b">{{ entry[2] }}</td>
          <td class="px-2 sm:px-4 py-1 sm:py-2 border-b">{{ entry[3] }}</td>
          <td class="px-2 sm:px-4 py-1 sm:py-2 border-b">{{ entry[4] }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { canister_contest_backend } from 'declarations/canister-contest-backend/index';

const leaderboard = ref([]);

const fetchLeaderboard = async () => {
  leaderboard.value = await canister_contest_backend.show();
};

onMounted(() => {
  fetchLeaderboard();
});
</script>

<style scoped>
table {
  width: 100%;
  border-collapse: collapse;
}
th, td {
  padding: 8px;
  border: 1px solid #ddd;
}
</style>
