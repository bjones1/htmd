export const publicPath = process.env.DEMO_PUBLIC_PATH ?? '/';

if (!/^\/(?:[a-zA-Z0-9_-]+\/)*$/.test(publicPath)) {
  throw new Error('DEMO_PUBLIC_PATH must be / or a path such as /htmd/');
}
