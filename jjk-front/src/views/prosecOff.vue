<script setup>
import { ref, onMounted } from 'vue';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import ColumnGroup from 'primevue/columngroup';   
import Row from 'primevue/row';                   
import Button from 'primevue/button';

//const products = [
//    { code: 'C001', name: 'Case A', category: 'Theft' },
//    { code: 'C002', name: 'Case B', category: 'Fraud' },
//    { code: 'C003', name: 'Case C', category: 'Assault' },
//];

const products = ref([]);
const showImage = ref(false);

onMounted(async () => {
  try {
    const response = await fetch('/jjk/rx/cases');
    const cases = await response.json();
    products.value = cases;
    console.log('Fetched cases:', cases);
  } catch (error) {
    console.error('Error fetching cases:', error);
  }
});

const downloadCase = async (caseCode) => {
  try {
    const response = await fetch(`/jjk/rx/download/${caseCode}`);
    const blob = await response.blob();
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${caseCode}.pdf`;
    a.click();
    window.URL.revokeObjectURL(url);
    showImage.value = true;
  } catch (error) {
    console.error('Error downloading case:', error);
  }
};
</script>

<template>
    <div class="justify-content-center text-center m-5">
  <h1 class="text-3xl font-bold tracking-tight text-heading md:text-4xl">Prosecutor's Office</h1>
  <h2 class="text-lg font-normal text-body lg:text-xl">List of available cases sent by the buffet</h2>

    <div class="justify-content-center text-center m-7">
        <DataTable :value="products" tableStyle="min-width: 50rem">
            <Column field="caseCode" header="Case Code"></Column>
            <Column field="description" header="Description"></Column>
            <Column field="createdAt" header="Created"></Column>
            <Column header="Download">
                <template #body="slotProps">
                    <Button label="Download" class="p-button-success" @click="downloadCase(slotProps.data.caseCode)"></Button>
                </template>
            </Column>
        </DataTable>
    </div>
    <transition name="fade-scale">
  <div v-if="showImage" class="flex flex-col items-center justify-center mt-4">
    <img src="/full.jpeg" alt="Download complete" class="max-w-full h-auto" />
    <p class="mt-4 text-lg font-semibold text-red-700">Acoplado</p>
  </div>
</transition>
</div>
</template>

<style scoped>
.fade-scale-enter-active {
  transition: all 0.6s ease-out;
}

.fade-scale-leave-active {
  transition: all 0.3s ease-in;
}

.fade-scale-enter-from {
  opacity: 0;
  transform: scale(0.8);
}

.fade-scale-leave-to {
  opacity: 0;
  transform: scale(0.9);
}
</style>
