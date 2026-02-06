<script setup>
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import ColumnGroup from 'primevue/columngroup';   
import Row from 'primevue/row';                   
import Button from 'primevue/button';

const products = [
    { code: 'C001', name: 'Case A', category: 'Theft' },
    { code: 'C002', name: 'Case B', category: 'Fraud' },
    { code: 'C003', name: 'Case C', category: 'Assault' },
];

</script>

<template>
    <div class="justify-content-center text-center m-5">
  <h1 class="text-3xl font-bold tracking-tight text-heading md:text-4xl">Prosecutor's Office</h1>
  <h2 class="text-lg font-normal text-body lg:text-xl">List of available cases sent by the buffet</h2>

    <div class="justify-content-center text-center m-7">
        <DataTable :value="products" tableStyle="min-width: 50rem">
            <Column field="code" header="Code"></Column>
            <Column field="name" header="Name"></Column>
            <Column field="category" header="Category"></Column>
            <Column header="Download">
                <template #body="slotProps">
                    <Button label="Download" class="p-button-success"></Button>
                </template>
            </Column>
        </DataTable>
    </div>
</div>
</template>