<template>
  <div id="app">
    <Calculator :a="3" :b="5">+</Calculator>
    <Calculator :a="3" :b="4">+</Calculator>
    <Calculator :a="2" :b="3">+</Calculator>
    <Calculator :a="-1" :b="0.9999">+</Calculator>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator'
import Calculator from './components/Calculator.vue'

@Component({ components: { Calculator } })
export default class App extends Vue {}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
