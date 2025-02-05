function set_local_date() {
    // Assume a date format of "2021-04-13T19:00:00+03:00";
    // Display time in localtime of the browser.

    const dates = document.getElementsByTagName("time");
    for (let ix=0; ix < dates.length; ix++) {
        //const mydate = dates[ix].getAttribute("x-schedule");
        const mydate = dates[ix].getAttribute("datetime");
        const date = new Date(mydate);

        dates[ix].innerHTML = date.toLocaleDateString( [], {
            weekday: 'long',
            year: 'numeric',
            month: 'long',
            day: 'numeric',
            hour: 'numeric',
            minute: 'numeric',
            timeZoneName: 'long'
        });
    }
}

function filter_events() {
  const now = new Date();
  console.log(`now: ${now}`);

  let days = document.getElementById("days").value;
  let start_hour_in_utc = parseInt(document.getElementById("start").value);
  let end_hour_in_utc = parseInt(document.getElementById("end").value);
  const selected_language = document.getElementById('language').value;
  console.log(`filter_events ${days} ${start_hour_in_utc}-${end_hour_in_utc} ${selected_language}`);

  config = {
      'days': days,
      'start': start_hour_in_utc,
      'end': end_hour_in_utc,
      'language': selected_language,
  };
  localStorage.setItem('config', JSON.stringify(config))

  //console.log(`start ${start_hour_in_utc}`);
  //console.log(`end ${end_hour_in_utc}`);

  //console.log(`filter_events ${days}`);

  document.getElementById("now").innerHTML = now.toLocaleDateString( [], {
            weekday: 'long',
            year: 'numeric',
            month: 'long',
            day: 'numeric',
            hour: 'numeric',
            minute: 'numeric',
            timeZoneName: 'long'
        });


  let cards = document.getElementsByClassName("card");
  document.getElementById("total").innerHTML = cards.length;

  let count = 0;
  for (let i = 0; i < cards.length; i++) {
    cards.item(i).style.display = "block";

    const language = cards.item(i).getElementsByClassName("language")[0].getAttribute("language")
    //console.log(language);
    if (selected_language != 'All') {
        if (selected_language != language) {
            cards.item(i).style.display = "none";
            continue;
        }
    }

    const datetime = cards.item(i).getElementsByTagName("time")[0].getAttribute("datetime")
    //console.log(datetime);
    const date = new Date(datetime);

    let event_start_hour_locally = (date.getUTCHours() - (now.getTimezoneOffset() / 60) + 24) % 24;
    //console.log(`event start locally: ${event_start_hour_locally}`);


    if ( event_start_hour_locally < start_hour_in_utc) {
        cards.item(i).style.display = "none";
        continue;
    }

    if (end_hour_in_utc < event_start_hour_locally) {
        cards.item(i).style.display = "none";
        continue;
    }

    let diff = datediff(now, date);
    //console.log(diff);

    if (diff < 0) {
        cards.item(i).style.display = "none";
        continue;
    }

    if (diff > days) {
        cards.item(i).style.display = "none";
        continue;
    }

    count++;
  }

  document.getElementById("showing").innerHTML = count;
}

function datediff(first, second) {
    return Math.round((second - first) / (1000 * 60 * 60 * 24));
}

function setup_languages() {
    const language_fields = document.getElementsByClassName("language");
    let languages = new Object;
    for (let ix=0; ix < language_fields.length; ix++) {
        const language = language_fields[ix].getAttribute("language");
        //console.log(language);
        if (! (language in languages)) {
            languages[language] = 0;
        }
        languages[language]++;
    }
    //console.log(languages);

    let language_selector = document.getElementById('language');
    let language_names = Object.keys(languages);

    let opt = document.createElement('option');
    opt.value = 'All';
    opt.innerHTML = 'All';
    language_selector.appendChild(opt);

    for (let i = 0; i < language_names.length; i++) {
        let opt = document.createElement('option');
        opt.value = language_names[i];
        opt.innerHTML = language_names[i];
        language_selector.appendChild(opt);
    }
}

function setup() {
  setup_languages();

  set_local_date();
  const config_str = localStorage.getItem('config');
  let config;
  if (config_str) {
      config = JSON.parse(config_str);
  } else {
      config = {
          'days': 14,
          'start': 7,
          'end': 22,
          'language': 'All',
      };
  }
  document.getElementById("days").value = config['days'];
  document.getElementById("start").value = config['start'];
  document.getElementById("end").value = config['end'];
  document.getElementById("language").value = config['language'];

  filter_events();
  document.getElementById("days").addEventListener("change", filter_events);
  document.getElementById("start").addEventListener("change", filter_events);
  document.getElementById("end").addEventListener("change", filter_events);
  document.getElementById("language").addEventListener("change", filter_events);
}

setup();

