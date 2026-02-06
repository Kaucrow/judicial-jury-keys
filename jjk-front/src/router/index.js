import { createRouter, createWebHistory } from 'vue-router';
import Buffet from '../views/buffet.vue';
import ProsecOff from '../views/prosecOff.vue';

const routes = [
  {
    path: '/buffet',
    name: 'buffet',
    component: Buffet
  },
  {
    path: '/prosecutor',
    name: 'prosecutor',
    component: ProsecOff
  },
  {
    path: '/',
    redirect: '/buffet' 
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;