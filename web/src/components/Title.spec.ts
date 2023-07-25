import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'

import Title from './Title.vue';

describe('Input', () => {
  it('renders properly', () => {
    // const wrapper = mount(Title, { props: { msg: 'Hello Vitest' } })
    const wrapper = mount(Title)
    expect(wrapper.text()).toContain('Todos')
  })
})
