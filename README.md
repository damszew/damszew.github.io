# damszew.dev

Personal blog for sharing thoughts about different practices and patters used to create clean and efficient software

Built with [Astro](https://astro.build) and the [AstroPaper](https://github.com/satnaing/astro-paper) theme.

## Development

The dev environment is managed by [devenv](https://devenv.sh). With `direnv` it loads automatically, otherwise run `devenv shell`.

```sh
dev     # start dev server
build   # type check, build and index for search into `dist/`
```

Posts live in `src/content/posts/`, the site config in `astro-paper.config.ts`.

## Deploy

Every push to `main` is built and deployed to GitHub Pages by `.github/workflows/cd.yml`.

## License

See the [LICENSE.md](LICENSE.md) file for details.
