<script setup>
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Button from 'primevue/button';
import Tag from 'primevue/tag'; 
import InputText from 'primevue/inputtext';
import { ref } from 'vue';

const products = ref([
    { title: 'Case A', subject: 'Penal', author: 'Jane Doe', keywords: ['theft', 'robbery', 'burglary'] },
    { title: 'Case B', subject: 'Penal', author: 'John Smith', keywords: ['fraud', 'scam'] },
    { title: 'Case C', subject: 'Penal', author: 'Alice Johnson', keywords: ['assault', 'violence'] },
]);

</script>

<template>
    <div class="justify-content-center text-center m-5">
  <h1 class="text-3xl font-bold tracking-tight text-heading md:text-4xl">Prosecutor's Office</h1>
  <h2 class="text-lg font-normal text-body lg:text-xl">List of available cases sent by the buffet</h2>

    <div class="justify-content-center text-center m-7">
        <DataTable :value="products" tableStyle="min-width: 50rem">
            <Column field="title" header="Title" sortable></Column>
            <Column field="subject" header="Subject" sortable></Column>
            <Column field="author" header="Author" sortable></Column>
            <Column field="keywords" header="Keywords">
                    <template #body="slotProps">
                        <div class="flex flex-wrap gap-1">
                            <Tag v-for="tag in slotProps.data.keywords" :key="tag" :value="tag" severity="secondary" />
                        </div>
                    </template>
                </Column>
            <Column header="Download">
                <template #body="slotProps">
                    <Button label="Download" class="p-button-success"></Button>
                    </template>
                </Column>
            </DataTable>
        </div>
    </div>
</template>