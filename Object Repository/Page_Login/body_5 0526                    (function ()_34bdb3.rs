<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_5 0526                    (function ()_34bdb3</name>
   <tag></tag>
   <elementGuidId>0de8d53a-6e04-4e1e-bdb0-5da95b029dd3</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>7e1ba9c2-91f9-40c9-b9d6-8805b800ff1d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

     
     

    

    
    
    
        
			        
    
    
        令和5年 05月26日 金曜日
               
    



    $(function () {

                var change_item = false;

        $('.browser').change(function() {
            change_item = true;
        });

        // 保存・実行ボタンからのポップアップ防止
        $('.btn-noty, .button').on('click', function(){
            var id = $(this).attr('id');
            if (typeof id !== &quot;undefined&quot;) {
                if (id.indexOf('back') == -1) { // 戻るボタンのみ有効
                    change_item = false;
                }
            }
        });

        $(window).on('beforeunload' ,function(e){
            if (change_item) {
                $('.wait-full-screen').css('display', '');
                return false;
            } else {
                e.stopPropagation();
            }
        });
        
        var stop_no_operation_timer = false;
        var time_limit_auto_save = 0;
        var time_limit = 1440;
        var no_operation_timer = setTimeout(timer_func, 60 * 1000);

        function timer_func() {

            
            
            if (stop_no_operation_timer === false) {
                no_operation_timer = setTimeout(timer_func, 60 * 1000);
            }
        }

        $('body').on('keydown mousedown', function () {
            time_limit_auto_save = 0;
            time_limit = 1440;
        });

        $('#holiday_master').on('click', function(e){
            e.preventDefault();
            var target_id = $(this).attr('id').split(&quot;_&quot;);
            var $form = $('&lt;form/>', {'action': '/Master/holiday/list', 'method': 'post'});
            $form.append($('&lt;input/>', {'type': 'hidden', 'name': 'nursery_id', 'value': ''}))
                .append($('&lt;input/>', {'type': 'hidden', 'name': 'jurisdiction_id', 'value': ''}))
                .appendTo(document.body);
            $form.submit();
        })

		
        var topBtn = $('.fixed-st');
        if (topBtn.length !== 0) {
            // ターゲットの位置取得
            var t = $('.fixed-st').offset().top + 50;
            var r = $('.fixed-st').offset().right;
            $(window).scroll(function () {
                if ($(this).scrollTop() > t) {
                    $('.fixed-st').addClass('fixed');
                } else {
                    $('.fixed-st').removeClass('fixed');
                }
            });
        }
    });



	.fixed{
		position: fixed;
		top: 1%;
		right: 2%;
	}

    

        
        
	


	
	
			
			

				ユーザーID
				
					
				

				
					
				

				
					パスワード
					
						
					
				

				
					
				

				
					ユーザーID入力
				

			
		

	



推奨環境
Microsoft Edge最新バージョン
Google Chrome最新バージョン
Mozilla Firefox最新バージョン
Safari最新バージョン





var flag_limitation = true;
var flag_passlimitation = true;
	$(function () {
		$('#login-id').blur(function () {
			if (flag_passlimitation) return;
			id_check($(this));
		});
	});

	$('#back_inputid').on('click', function(e){
        e.preventDefault();
		flag_limitation = true;
		$('div[name=&quot;input_pass&quot;').css('display', 'none');
		$('div[name=&quot;input_id&quot;').css('display', 'block');
		$('#title_id').html('ユーザーID');
		$('#errors').html('');
    });

	$('input[name=&quot;next&quot;]').on('click', function(e){
        e.preventDefault();
		id_check($('#login-id'));
		$('#password').val('');
    });
	document.onkeyup = function(e) {
		if(e.keyCode == 13) {
			if(flag_limitation){
				$('input[name=&quot;next&quot;]').trigger(&quot;click&quot;);
			}else{
				$('input[name=&quot;login&quot;]').trigger(&quot;click&quot;);
			}
		}
	}

	// ログインユーザー入力時
	$(&quot;#login-id&quot;).keydown(function(e){
		// エンターキー押下
		if(e.which == 13){
			$('input[name=&quot;next&quot;]').click();
		}
	});

	// パスワード入力時
	$(&quot;#password&quot;).keydown(function(e){
		// エンターキー押下
		if(e.which == 13){
			$('#login-form').submit()
		}
	});

	function id_check (id_val) {
		var wait_screen = $('.wait-full-screen');
		if (id_val.val() != '') {
			wait_screen.css('display', 'block');
			$('#errors').html('');
			$.ajax({
				url: '/Login/sendMessage'
				, type: 'POST'
				, dataType: 'json'
				, data: {
					'item': id_val.val()
				}
			})
				.done(
					function(data, textStatus, jqXHR ) {
						if (data.status =='success') {
							flag_limitation = false;
							if (data.posted_data == null) {
								noty({
									text: 'パスワードを送信しました。',
									layout: 'topRight',
									type: 'success',
									closeWith: ['click', 'button'],
									animation: {open: 'animated zoomIn', close: 'animated zoomOut'}
								});
							} else if (data.posted_data == 'debug') {
							} else if (data.posted_data == 'fix') {
							} else {
								$('#login-button').prop('disabled', true);
								$('.maintenance-screen')
									.css(&quot;display&quot;, &quot;block&quot;)
									.html('現在、メンテナンスを行っております。&lt;br>終了時刻は'+data.posted_data+'を予定しております。')
							}

							if (flag_passlimitation) {
								$('#title_id').html('ユーザーID' + ' : ' + data.info.user_id)
								$('div[name=&quot;input_pass&quot;').css('display', 'block');
								$('div[name=&quot;input_id&quot;').css('display', 'none');
							}
						} else {

							noty({
								text: 'ユーザIDが見つかりません。',
								layout: 'topRight',
								type: 'success',
								closeWith: ['click', 'button'],
								animation: {open: 'animated zoomIn', close: 'animated zoomOut'}
							});

						}
					}
				)
				.error(
					function (data, textStatus, jqXHR ) {
						alert('正常に通信が出来ません、システム管理者に問い合わせてください。');
					}
				)
				.always(
					function (data, textStatus, jqXHR ) {
						wait_screen.css('display', 'none');
					}
				)
		} else {
			if (flag_passlimitation) {
				$('#errors').html('ユーザーIDが入力されていません');
			}
		}
	}
    

    
    おが〜るウェブレポ 1.3.166271.18(2.10.11)® - © 2013 - 2023 Iwate Information Technology, Inc. All rights reserved.Powered by おが〜るシステム


    $(document).ready(function () {
        $('.wait-full-screen').css('display', 'none');
    });



id(&quot;login-id&quot;)</value>
      <webElementGuid>7763d692-39d0-4343-9fc4-189545fd704e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>56e014b3-bd2c-4a32-afd4-989765af9f04</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>5c7265f1-7aea-4d08-a007-33df84aaf943</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;

     
     

    

    
    
    
        
			        
    
    
        令和5年 05月26日 金曜日
               
    



    $(function () {

                var change_item = false;

        $(&quot; , &quot;'&quot; , &quot;.browser&quot; , &quot;'&quot; , &quot;).change(function() {
            change_item = true;
        });

        // 保存・実行ボタンからのポップアップ防止
        $(&quot; , &quot;'&quot; , &quot;.btn-noty, .button&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(){
            var id = $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;);
            if (typeof id !== &quot;undefined&quot;) {
                if (id.indexOf(&quot; , &quot;'&quot; , &quot;back&quot; , &quot;'&quot; , &quot;) == -1) { // 戻るボタンのみ有効
                    change_item = false;
                }
            }
        });

        $(window).on(&quot; , &quot;'&quot; , &quot;beforeunload&quot; , &quot;'&quot; , &quot; ,function(e){
            if (change_item) {
                $(&quot; , &quot;'&quot; , &quot;.wait-full-screen&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                return false;
            } else {
                e.stopPropagation();
            }
        });
        
        var stop_no_operation_timer = false;
        var time_limit_auto_save = 0;
        var time_limit = 1440;
        var no_operation_timer = setTimeout(timer_func, 60 * 1000);

        function timer_func() {

            
            
            if (stop_no_operation_timer === false) {
                no_operation_timer = setTimeout(timer_func, 60 * 1000);
            }
        }

        $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;keydown mousedown&quot; , &quot;'&quot; , &quot;, function () {
            time_limit_auto_save = 0;
            time_limit = 1440;
        });

        $(&quot; , &quot;'&quot; , &quot;#holiday_master&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
            e.preventDefault();
            var target_id = $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;).split(&quot;_&quot;);
            var $form = $(&quot; , &quot;'&quot; , &quot;&lt;form/>&quot; , &quot;'&quot; , &quot;, {&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;/Master/holiday/list&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;method&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;post&quot; , &quot;'&quot; , &quot;});
            $form.append($(&quot; , &quot;'&quot; , &quot;&lt;input/>&quot; , &quot;'&quot; , &quot;, {&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;nursery_id&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;}))
                .append($(&quot; , &quot;'&quot; , &quot;&lt;input/>&quot; , &quot;'&quot; , &quot;, {&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;jurisdiction_id&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;}))
                .appendTo(document.body);
            $form.submit();
        })

		
        var topBtn = $(&quot; , &quot;'&quot; , &quot;.fixed-st&quot; , &quot;'&quot; , &quot;);
        if (topBtn.length !== 0) {
            // ターゲットの位置取得
            var t = $(&quot; , &quot;'&quot; , &quot;.fixed-st&quot; , &quot;'&quot; , &quot;).offset().top + 50;
            var r = $(&quot; , &quot;'&quot; , &quot;.fixed-st&quot; , &quot;'&quot; , &quot;).offset().right;
            $(window).scroll(function () {
                if ($(this).scrollTop() > t) {
                    $(&quot; , &quot;'&quot; , &quot;.fixed-st&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;fixed&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot; , &quot;'&quot; , &quot;.fixed-st&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;fixed&quot; , &quot;'&quot; , &quot;);
                }
            });
        }
    });



	.fixed{
		position: fixed;
		top: 1%;
		right: 2%;
	}

    

        
        
	


	
	
			
			

				ユーザーID
				
					
				

				
					
				

				
					パスワード
					
						
					
				

				
					
				

				
					ユーザーID入力
				

			
		

	



推奨環境
Microsoft Edge最新バージョン
Google Chrome最新バージョン
Mozilla Firefox最新バージョン
Safari最新バージョン





var flag_limitation = true;
var flag_passlimitation = true;
	$(function () {
		$(&quot; , &quot;'&quot; , &quot;#login-id&quot; , &quot;'&quot; , &quot;).blur(function () {
			if (flag_passlimitation) return;
			id_check($(this));
		});
	});

	$(&quot; , &quot;'&quot; , &quot;#back_inputid&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
        e.preventDefault();
		flag_limitation = true;
		$(&quot; , &quot;'&quot; , &quot;div[name=&quot;input_pass&quot;&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;div[name=&quot;input_id&quot;&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#title_id&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;ユーザーID&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#errors&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    });

	$(&quot; , &quot;'&quot; , &quot;input[name=&quot;next&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
        e.preventDefault();
		id_check($(&quot; , &quot;'&quot; , &quot;#login-id&quot; , &quot;'&quot; , &quot;));
		$(&quot; , &quot;'&quot; , &quot;#password&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    });
	document.onkeyup = function(e) {
		if(e.keyCode == 13) {
			if(flag_limitation){
				$(&quot; , &quot;'&quot; , &quot;input[name=&quot;next&quot;]&quot; , &quot;'&quot; , &quot;).trigger(&quot;click&quot;);
			}else{
				$(&quot; , &quot;'&quot; , &quot;input[name=&quot;login&quot;]&quot; , &quot;'&quot; , &quot;).trigger(&quot;click&quot;);
			}
		}
	}

	// ログインユーザー入力時
	$(&quot;#login-id&quot;).keydown(function(e){
		// エンターキー押下
		if(e.which == 13){
			$(&quot; , &quot;'&quot; , &quot;input[name=&quot;next&quot;]&quot; , &quot;'&quot; , &quot;).click();
		}
	});

	// パスワード入力時
	$(&quot;#password&quot;).keydown(function(e){
		// エンターキー押下
		if(e.which == 13){
			$(&quot; , &quot;'&quot; , &quot;#login-form&quot; , &quot;'&quot; , &quot;).submit()
		}
	});

	function id_check (id_val) {
		var wait_screen = $(&quot; , &quot;'&quot; , &quot;.wait-full-screen&quot; , &quot;'&quot; , &quot;);
		if (id_val.val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
			wait_screen.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
			$(&quot; , &quot;'&quot; , &quot;#errors&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
			$.ajax({
				url: &quot; , &quot;'&quot; , &quot;/Login/sendMessage&quot; , &quot;'&quot; , &quot;
				, type: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;
				, dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;
				, data: {
					&quot; , &quot;'&quot; , &quot;item&quot; , &quot;'&quot; , &quot;: id_val.val()
				}
			})
				.done(
					function(data, textStatus, jqXHR ) {
						if (data.status ==&quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;) {
							flag_limitation = false;
							if (data.posted_data == null) {
								noty({
									text: &quot; , &quot;'&quot; , &quot;パスワードを送信しました。&quot; , &quot;'&quot; , &quot;,
									layout: &quot; , &quot;'&quot; , &quot;topRight&quot; , &quot;'&quot; , &quot;,
									type: &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;,
									closeWith: [&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot;],
									animation: {open: &quot; , &quot;'&quot; , &quot;animated zoomIn&quot; , &quot;'&quot; , &quot;, close: &quot; , &quot;'&quot; , &quot;animated zoomOut&quot; , &quot;'&quot; , &quot;}
								});
							} else if (data.posted_data == &quot; , &quot;'&quot; , &quot;debug&quot; , &quot;'&quot; , &quot;) {
							} else if (data.posted_data == &quot; , &quot;'&quot; , &quot;fix&quot; , &quot;'&quot; , &quot;) {
							} else {
								$(&quot; , &quot;'&quot; , &quot;#login-button&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
								$(&quot; , &quot;'&quot; , &quot;.maintenance-screen&quot; , &quot;'&quot; , &quot;)
									.css(&quot;display&quot;, &quot;block&quot;)
									.html(&quot; , &quot;'&quot; , &quot;現在、メンテナンスを行っております。&lt;br>終了時刻は&quot; , &quot;'&quot; , &quot;+data.posted_data+&quot; , &quot;'&quot; , &quot;を予定しております。&quot; , &quot;'&quot; , &quot;)
							}

							if (flag_passlimitation) {
								$(&quot; , &quot;'&quot; , &quot;#title_id&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;ユーザーID&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot; + data.info.user_id)
								$(&quot; , &quot;'&quot; , &quot;div[name=&quot;input_pass&quot;&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
								$(&quot; , &quot;'&quot; , &quot;div[name=&quot;input_id&quot;&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
							}
						} else {

							noty({
								text: &quot; , &quot;'&quot; , &quot;ユーザIDが見つかりません。&quot; , &quot;'&quot; , &quot;,
								layout: &quot; , &quot;'&quot; , &quot;topRight&quot; , &quot;'&quot; , &quot;,
								type: &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;,
								closeWith: [&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot;],
								animation: {open: &quot; , &quot;'&quot; , &quot;animated zoomIn&quot; , &quot;'&quot; , &quot;, close: &quot; , &quot;'&quot; , &quot;animated zoomOut&quot; , &quot;'&quot; , &quot;}
							});

						}
					}
				)
				.error(
					function (data, textStatus, jqXHR ) {
						alert(&quot; , &quot;'&quot; , &quot;正常に通信が出来ません、システム管理者に問い合わせてください。&quot; , &quot;'&quot; , &quot;);
					}
				)
				.always(
					function (data, textStatus, jqXHR ) {
						wait_screen.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
					}
				)
		} else {
			if (flag_passlimitation) {
				$(&quot; , &quot;'&quot; , &quot;#errors&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;ユーザーIDが入力されていません&quot; , &quot;'&quot; , &quot;);
			}
		}
	}
    

    
    おが〜るウェブレポ 1.3.166271.18(2.10.11)® - © 2013 - 2023 Iwate Information Technology, Inc. All rights reserved.Powered by おが〜るシステム


    $(document).ready(function () {
        $(&quot; , &quot;'&quot; , &quot;.wait-full-screen&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
    });



id(&quot;login-id&quot;)&quot;) or . = concat(&quot;

     
     

    

    
    
    
        
			        
    
    
        令和5年 05月26日 金曜日
               
    



    $(function () {

                var change_item = false;

        $(&quot; , &quot;'&quot; , &quot;.browser&quot; , &quot;'&quot; , &quot;).change(function() {
            change_item = true;
        });

        // 保存・実行ボタンからのポップアップ防止
        $(&quot; , &quot;'&quot; , &quot;.btn-noty, .button&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(){
            var id = $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;);
            if (typeof id !== &quot;undefined&quot;) {
                if (id.indexOf(&quot; , &quot;'&quot; , &quot;back&quot; , &quot;'&quot; , &quot;) == -1) { // 戻るボタンのみ有効
                    change_item = false;
                }
            }
        });

        $(window).on(&quot; , &quot;'&quot; , &quot;beforeunload&quot; , &quot;'&quot; , &quot; ,function(e){
            if (change_item) {
                $(&quot; , &quot;'&quot; , &quot;.wait-full-screen&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                return false;
            } else {
                e.stopPropagation();
            }
        });
        
        var stop_no_operation_timer = false;
        var time_limit_auto_save = 0;
        var time_limit = 1440;
        var no_operation_timer = setTimeout(timer_func, 60 * 1000);

        function timer_func() {

            
            
            if (stop_no_operation_timer === false) {
                no_operation_timer = setTimeout(timer_func, 60 * 1000);
            }
        }

        $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;keydown mousedown&quot; , &quot;'&quot; , &quot;, function () {
            time_limit_auto_save = 0;
            time_limit = 1440;
        });

        $(&quot; , &quot;'&quot; , &quot;#holiday_master&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
            e.preventDefault();
            var target_id = $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;).split(&quot;_&quot;);
            var $form = $(&quot; , &quot;'&quot; , &quot;&lt;form/>&quot; , &quot;'&quot; , &quot;, {&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;/Master/holiday/list&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;method&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;post&quot; , &quot;'&quot; , &quot;});
            $form.append($(&quot; , &quot;'&quot; , &quot;&lt;input/>&quot; , &quot;'&quot; , &quot;, {&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;nursery_id&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;}))
                .append($(&quot; , &quot;'&quot; , &quot;&lt;input/>&quot; , &quot;'&quot; , &quot;, {&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;jurisdiction_id&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;}))
                .appendTo(document.body);
            $form.submit();
        })

		
        var topBtn = $(&quot; , &quot;'&quot; , &quot;.fixed-st&quot; , &quot;'&quot; , &quot;);
        if (topBtn.length !== 0) {
            // ターゲットの位置取得
            var t = $(&quot; , &quot;'&quot; , &quot;.fixed-st&quot; , &quot;'&quot; , &quot;).offset().top + 50;
            var r = $(&quot; , &quot;'&quot; , &quot;.fixed-st&quot; , &quot;'&quot; , &quot;).offset().right;
            $(window).scroll(function () {
                if ($(this).scrollTop() > t) {
                    $(&quot; , &quot;'&quot; , &quot;.fixed-st&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;fixed&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot; , &quot;'&quot; , &quot;.fixed-st&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;fixed&quot; , &quot;'&quot; , &quot;);
                }
            });
        }
    });



	.fixed{
		position: fixed;
		top: 1%;
		right: 2%;
	}

    

        
        
	


	
	
			
			

				ユーザーID
				
					
				

				
					
				

				
					パスワード
					
						
					
				

				
					
				

				
					ユーザーID入力
				

			
		

	



推奨環境
Microsoft Edge最新バージョン
Google Chrome最新バージョン
Mozilla Firefox最新バージョン
Safari最新バージョン





var flag_limitation = true;
var flag_passlimitation = true;
	$(function () {
		$(&quot; , &quot;'&quot; , &quot;#login-id&quot; , &quot;'&quot; , &quot;).blur(function () {
			if (flag_passlimitation) return;
			id_check($(this));
		});
	});

	$(&quot; , &quot;'&quot; , &quot;#back_inputid&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
        e.preventDefault();
		flag_limitation = true;
		$(&quot; , &quot;'&quot; , &quot;div[name=&quot;input_pass&quot;&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;div[name=&quot;input_id&quot;&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#title_id&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;ユーザーID&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#errors&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    });

	$(&quot; , &quot;'&quot; , &quot;input[name=&quot;next&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
        e.preventDefault();
		id_check($(&quot; , &quot;'&quot; , &quot;#login-id&quot; , &quot;'&quot; , &quot;));
		$(&quot; , &quot;'&quot; , &quot;#password&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    });
	document.onkeyup = function(e) {
		if(e.keyCode == 13) {
			if(flag_limitation){
				$(&quot; , &quot;'&quot; , &quot;input[name=&quot;next&quot;]&quot; , &quot;'&quot; , &quot;).trigger(&quot;click&quot;);
			}else{
				$(&quot; , &quot;'&quot; , &quot;input[name=&quot;login&quot;]&quot; , &quot;'&quot; , &quot;).trigger(&quot;click&quot;);
			}
		}
	}

	// ログインユーザー入力時
	$(&quot;#login-id&quot;).keydown(function(e){
		// エンターキー押下
		if(e.which == 13){
			$(&quot; , &quot;'&quot; , &quot;input[name=&quot;next&quot;]&quot; , &quot;'&quot; , &quot;).click();
		}
	});

	// パスワード入力時
	$(&quot;#password&quot;).keydown(function(e){
		// エンターキー押下
		if(e.which == 13){
			$(&quot; , &quot;'&quot; , &quot;#login-form&quot; , &quot;'&quot; , &quot;).submit()
		}
	});

	function id_check (id_val) {
		var wait_screen = $(&quot; , &quot;'&quot; , &quot;.wait-full-screen&quot; , &quot;'&quot; , &quot;);
		if (id_val.val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
			wait_screen.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
			$(&quot; , &quot;'&quot; , &quot;#errors&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
			$.ajax({
				url: &quot; , &quot;'&quot; , &quot;/Login/sendMessage&quot; , &quot;'&quot; , &quot;
				, type: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;
				, dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;
				, data: {
					&quot; , &quot;'&quot; , &quot;item&quot; , &quot;'&quot; , &quot;: id_val.val()
				}
			})
				.done(
					function(data, textStatus, jqXHR ) {
						if (data.status ==&quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;) {
							flag_limitation = false;
							if (data.posted_data == null) {
								noty({
									text: &quot; , &quot;'&quot; , &quot;パスワードを送信しました。&quot; , &quot;'&quot; , &quot;,
									layout: &quot; , &quot;'&quot; , &quot;topRight&quot; , &quot;'&quot; , &quot;,
									type: &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;,
									closeWith: [&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot;],
									animation: {open: &quot; , &quot;'&quot; , &quot;animated zoomIn&quot; , &quot;'&quot; , &quot;, close: &quot; , &quot;'&quot; , &quot;animated zoomOut&quot; , &quot;'&quot; , &quot;}
								});
							} else if (data.posted_data == &quot; , &quot;'&quot; , &quot;debug&quot; , &quot;'&quot; , &quot;) {
							} else if (data.posted_data == &quot; , &quot;'&quot; , &quot;fix&quot; , &quot;'&quot; , &quot;) {
							} else {
								$(&quot; , &quot;'&quot; , &quot;#login-button&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
								$(&quot; , &quot;'&quot; , &quot;.maintenance-screen&quot; , &quot;'&quot; , &quot;)
									.css(&quot;display&quot;, &quot;block&quot;)
									.html(&quot; , &quot;'&quot; , &quot;現在、メンテナンスを行っております。&lt;br>終了時刻は&quot; , &quot;'&quot; , &quot;+data.posted_data+&quot; , &quot;'&quot; , &quot;を予定しております。&quot; , &quot;'&quot; , &quot;)
							}

							if (flag_passlimitation) {
								$(&quot; , &quot;'&quot; , &quot;#title_id&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;ユーザーID&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot; + data.info.user_id)
								$(&quot; , &quot;'&quot; , &quot;div[name=&quot;input_pass&quot;&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
								$(&quot; , &quot;'&quot; , &quot;div[name=&quot;input_id&quot;&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
							}
						} else {

							noty({
								text: &quot; , &quot;'&quot; , &quot;ユーザIDが見つかりません。&quot; , &quot;'&quot; , &quot;,
								layout: &quot; , &quot;'&quot; , &quot;topRight&quot; , &quot;'&quot; , &quot;,
								type: &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;,
								closeWith: [&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot;],
								animation: {open: &quot; , &quot;'&quot; , &quot;animated zoomIn&quot; , &quot;'&quot; , &quot;, close: &quot; , &quot;'&quot; , &quot;animated zoomOut&quot; , &quot;'&quot; , &quot;}
							});

						}
					}
				)
				.error(
					function (data, textStatus, jqXHR ) {
						alert(&quot; , &quot;'&quot; , &quot;正常に通信が出来ません、システム管理者に問い合わせてください。&quot; , &quot;'&quot; , &quot;);
					}
				)
				.always(
					function (data, textStatus, jqXHR ) {
						wait_screen.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
					}
				)
		} else {
			if (flag_passlimitation) {
				$(&quot; , &quot;'&quot; , &quot;#errors&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;ユーザーIDが入力されていません&quot; , &quot;'&quot; , &quot;);
			}
		}
	}
    

    
    おが〜るウェブレポ 1.3.166271.18(2.10.11)® - © 2013 - 2023 Iwate Information Technology, Inc. All rights reserved.Powered by おが〜るシステム


    $(document).ready(function () {
        $(&quot; , &quot;'&quot; , &quot;.wait-full-screen&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
    });



id(&quot;login-id&quot;)&quot;))]</value>
      <webElementGuid>916d2627-7a38-4d25-b9ec-66a5b403b846</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
