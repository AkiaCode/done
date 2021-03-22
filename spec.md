### Spec


* import
```javascript
import javaScript from 'done'
import javaScript1 from 'done@0.1.0'
import javaScript2 from 'https:///example.com.js'
import javaScript3 from '<url>'
import javaScript4 from './sdasda/done.js'
```

* Disable flag
```bash
done run ./done.js
> You want read File? Y
starting
# or
done run -A ./done.js
> Warning : <adadsafsaaf>
starting
```

* download module
```bash
done module install done
> Installing....
# or
done module install https:///example.com.js
> Installing....
# or
done module install done@1.0
> Installing....
```

* Support Web API (100%)
* Support plugin api
* Event Loop (tokie)
* Support Promise (async, await)
* Remove node_modules, pacakges.json