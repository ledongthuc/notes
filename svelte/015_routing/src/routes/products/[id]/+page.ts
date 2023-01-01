import { error } from '@sveltejs/kit';
 
/** @type {import('./$types').PageLoad} */
export function load({ params }) {
  if (params.id === '1') {
    return {
      title: 'Product 1',
      content: 'Welcome to product 1. Lorem ipsum dolor sit amet...'
    };
  }
 
  throw error(404, 'Not found');
}