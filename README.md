# Instructions
On your machine, install libssl-dev
```bash
sudo apt install libssl-dev
```

# Options
* Add Header Option
* Add Body Option
* Search only for Same Domains
* Max-Height (default is 3 - Maximum is 10)


# Docs
## Redirect Pattern
In the following list, are all the types of redirect that this program can find. For now, only `JS` and `HTML` default methods.
```html
<a ... href="REDIRECT"> </a>
<script ... src="PATH"> </script>
```

```js
location.href = "REDIRECT";
location.replace("REDIRECT");
location.assign("REDIRECT");
location.pathname = "REDIRECT";
```
