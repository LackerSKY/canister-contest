<template>
  <div class="p-8 max-w-4xl mx-auto bg-white rounded-lg shadow-lg space-y-6">
    <h2 class="text-3xl font-bold text-center text-gray-800">Leaderboard</h2>
    <table class="w-full border-collapse">
      <thead>
        <tr>
          <th class="px-4 py-2 border-b text-left">Player 1</th>
          <th class="px-4 py-2 border-b text-left">Score</th>
          <th class="px-4 py-2 border-b text-left">Player 2</th>
          <th class="px-4 py-2 border-b text-left">Score</th>
          <th class="px-4 py-2 border-b text-left">Score difference</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(entry, index) in leaderboard" :key="index">
          <td class="px-4 py-2 border-b">{{ entry[0] }}</td>
          <td class="px-4 py-2 border-b">{{ entry[1] }}</td>
          <td class="px-4 py-2 border-b">{{ entry[2] }}</td>
          <td class="px-4 py-2 border-b">{{ entry[3] }}</td>
          <td class="px-4 py-2 border-b">{{ entry[4] }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { canister_contest_backend} from 'declarations/canister-contest-backend/index';

const leaderboard = ref([]);

const fetchLeaderboard = async () => {
  leaderboard.value = await canister_contest_backend.read_leaderboard();
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
