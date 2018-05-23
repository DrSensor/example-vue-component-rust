<template>
  <div>{{a}} <slot/> {{b}} = {{result}}</div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator'
import loadWasm from '@/libs/calculator.rs'

@Component
export default class Led extends Vue {
  @Prop() a!: number
  @Prop() b!: number

  beforeCreate() {
    loadWasm().then(result => {
      this.calculate = result.instance.exports.add
    })
  }

  private calculate = (a: number, b: number) => Number()

  get result() {
    return this.calculate(this.a, this.b)
  }
}
</script>

<style scoped>
</style>
